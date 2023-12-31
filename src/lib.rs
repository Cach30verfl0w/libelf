#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod endian;
pub mod header;
#[cfg(test)] pub mod tests;

use compile_warning::compile_warning;
use thiserror_no_std::Error;

#[cfg(feature = "std")] pub use std;

#[cfg(not(feature = "std"))] use alloc::vec::Vec;

use crate::{
    header::{
        ident::ElfIdent,
        FileHeader,
        ProgramHeader,
        SectionHeader,
    },
    std::mem::size_of,
};
#[cfg(not(feature = "std"))] pub use core as std;

// Inform a potential user that this library is not intended for use in production environments.
// This is for the reason that this project is only a project so that I can get more familiar with
// the ELF format and understand the specification better. So I ask anyone who wants to use this to
// use a different library.
compile_warning!(This library is not suitable for production usage);

/// This enum represents all possible recoverable error codes in this library. This error is used on
/// any operation that can fail.
#[derive(Error, Debug)]
pub enum Error {
    /// The ELF magic bytes can't be found in the specified ELF data or data holder
    #[error("Unable to find magic bytes in specified ELF")]
    InvalidMagic,

    /// The specified ELF data's size is not high enough to be a ELF file
    #[error("The size {0} is too low for an ELF file, please check your parameters")]
    NotEnoughBytes(usize),

    /// Some std I/O operation fails (Only available with `std`-feature)
    #[error(transparent)]
    #[cfg(feature = "std")]
    IO(#[from] std::io::Error),

    /// The provided ELF file's class is not valid
    #[error("The provided ELF file's class is not valid")]
    InvalidClass,
}

pub struct Elf<'a> {
    header: FileHeader,
    program_headers: Option<Vec<ProgramHeader>>,
    section_headers: Option<Vec<SectionHeader>>,
    bytes: &'a [u8],
}

impl<'a> Elf<'a> {
    /// This field contains the magic bytes of an ELF file
    const MAGIC_BYTES: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];

    /// This field contains the minimal size of an ELF file
    const MIN_ELF_SIZE: usize = size_of::<ElfIdent>();

    /// This function accepts a byte slice and parses it into the content of the ELF file. But this
    /// conversion can fail, if the validation of the values in the header or other section data is
    /// invalid.
    ///
    /// Here is a list with all errors, which can occur while this operation:
    /// - [Error::InvalidMagic] - The magic bytes of the file can't be found
    /// - [Error::NotEnoughBytes] - The specified ELF data's size is not high enough to be a ELF file
    /// - [Error::InvalidClass] - The provided ELF file's class is not valid
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, Error> {
        // Get index of ELF header and validate size of the file with magic bytes index as start
        // point
        let index = Self::elf_index(bytes).ok_or(Error::InvalidMagic)? + 4;
        if (bytes.len() - index) < Self::MIN_ELF_SIZE {
            return Err(Error::NotEnoughBytes(bytes.len() - index));
        }

        // Read ELF header
        let header = FileHeader::read(bytes, index.clone())?;

        // Read all program headers
        let program_headers = if header.program_header_count > 0 {
            let mut program_headers = Vec::new();
            for i in 0..header.section_header_count {
                program_headers.push(ProgramHeader::read(
                    &header.ident,
                    bytes,
                    index - 4
                        + header.program_header_offset as usize
                        + (i * header.program_header_size) as usize,
                )?);
            }
            Some(program_headers)
        } else {
            None
        };

        // Read all section headers
        let section_headers = if header.section_header_count > 0 {
            let mut section_headers = Vec::new();
            for i in 0..header.section_header_count {
                section_headers.push(SectionHeader::read(
                    &header.ident,
                    bytes,
                    index - 4
                        + header.section_header_offset as usize
                        + (i * header.section_header_size) as usize,
                )?);
            }
            Some(section_headers)
        } else {
            None
        };

        // Return parsed, validated and prepared ELF structure
        Ok(Elf {
            header,
            program_headers,
            section_headers,
            bytes: &bytes[(index - 4)..bytes.len()],
        })
    }

    /// This function scans the specified data for the ELF magic bytes. If no magic bytes are found
    /// the function returns a None. Otherwise this function returns the index of the magic bytes in
    /// the specified data.
    fn elf_index(bytes: &[u8]) -> Option<usize> {
        for i in 0..(bytes.len() - Self::MAGIC_BYTES.len()) {
            if bytes[i..=(i + 3)].eq(Self::MAGIC_BYTES.as_slice()) {
                return Some(i);
            }
        }
        None
    }

    /// This function returns a reference to the file header.
    #[inline]
    pub const fn file_header(&self) -> &FileHeader {
        &self.header
    }

    #[inline]
    pub const fn program_headers(&self) -> Option<&Vec<ProgramHeader>> {
        self.program_headers.as_ref()
    }

    #[inline]
    pub const fn section_headers(&self) -> Option<&Vec<SectionHeader>> {
        self.section_headers.as_ref()
    }
}
