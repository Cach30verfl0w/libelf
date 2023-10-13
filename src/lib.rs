#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod tests;
pub mod header;

use compile_warning::compile_warning;
use thiserror_no_std::Error;

#[cfg(feature = "std")]
pub use std;

#[cfg(not(feature = "std"))]
pub use core as std;
use std::mem::{size_of, transmute};
use crate::header::FileHeader;
use crate::header::ident::ElfIdent;

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
    IO(#[from] std::io::Error)
}

pub struct Elf {
    header: FileHeader
}

impl Elf {
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
    pub fn from_bytes(bytes: &[u8]) -> Result<Elf, Error> {
        // Get index of ELF header and validate size of the file with magic bytes index as start
        // point
        let index = Self::elf_index(bytes).ok_or(Error::InvalidMagic)? + 4;
        if (bytes.len() - index) < Self::MIN_ELF_SIZE {
            return Err(Error::NotEnoughBytes(bytes.len() - index));
        }

        // Convert bytes in slice of header to ident bytes TODO: Use safe transmute when available
        let ident: ElfIdent = unsafe {
            let mut ident_data = [0; size_of::<ElfIdent>()];
            ident_data.copy_from_slice(&bytes[index..(index + size_of::<ElfIdent>())]);
            transmute(ident_data)
        };

        // Return parsed, validated and prepared ELF structure
        Ok(Elf {
            header: FileHeader {
                ident
            },
        })
    }

    /// This function accepts the specified path, opens the file and reads the content into a byte
    /// slice. The byte slice is given to the [Elf::from_bytes] function.
    ///
    /// **This function uses heap allocations to read the file into a in-memory structure**
    ///
    /// Here is a list with all errors, which can occur while this operation:
    /// - [Error::InvalidMagic] - The magic bytes of the file can't be found
    /// - [Error::IO] - Some std I/O operation fails (Only available with `std`-feature)
    /// - [Error::NotEnoughBytes] - The specified ELF file's is not big enough to be a ELF file
    #[inline(always)]
    #[cfg(feature = "std")]
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Elf, Error> {
        use std::{fs::File, io::Read};

        // Read file into bytes
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        let length = file.read_to_end(&mut bytes)?;

        // Validate file length
        if length < Self::MIN_ELF_SIZE {
            return Err(Error::NotEnoughBytes(length));
        }

        // Give from_bytes to bytes as slice
        Self::from_bytes(bytes.as_slice())
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
    fn file_header(&self) -> &FileHeader {
        &self.header
    }

}