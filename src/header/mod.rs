use std::mem;
use crate::Error;
use crate::header::ident::{ElfClass, ElfIdent};

pub mod ident;

/// This enum represents the type of the ELF file. The file can be a relocatable file, an executable
/// file, an shared object or an core file.
///
/// - [FileType::None]: No file type defined
/// - [FileType::Relocatable]: Relocatable file
/// - [FileType::Executable]: Executable file
/// - [FileType::SharedObject]: Shared Object file#
/// - [FileType::Core]: Core file
///
/// ## See also
/// - [ELF Header](https://www.sco.com/developers/gabi/latest/ch4.eheader.html) by SCO, Inc.
#[repr(u16)]
#[rustfmt::skip]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum FileType {
    #[default]
    None         = 0,
    Relocatable  = 1,
    Executable   = 2,
    SharedObject = 3,
    Core         = 4
}

impl From<u16> for FileType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Relocatable,
            2 => Self::Executable,
            3 => Self::SharedObject,
            4 => Self::Core,
            _ => Self::None
        }
    }
}

#[repr(u16)]
#[rustfmt::skip]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum TargetMachine {
    #[default]
    None   = 0,
    X86_64 = 62,
    ARM    = 40,
    ARM64  = 183,
    RISCV  = 243
}

impl From<u16> for TargetMachine {
    fn from(value: u16) -> Self {
        match value {
            62 => Self::X86_64,
            40 => Self::ARM,
            183 => Self::ARM64,
            243 => Self::RISCV,
            _ => Self::None
        }
    }
}

/// This struct represents the file header of an ELF file. This header contains information about
/// the different program and section headers and the location of them in the file. We can also read
/// information about the file.
///
/// Here is a list with the fields of the header:
/// - `FileHeader::ident` - The identification bytes of the ELF file (without magic bytes)
///
/// ## See also
/// - [ELF Header](https://www.sco.com/developers/gabi/latest/ch4.eheader.html) by SCO, Inc.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct FileHeader {
    /// This struct represents the indication bytes of the ELF file without the ELF magic bytes. For
    /// more information, see [ElfIdent].
    pub ident: ElfIdent,

    /// This field represents the type of the ELF file. For more information, see [FileType].
    pub ty: FileType,

    /// This field represents the architecture target of the ELF file. For more information, see
    /// [TargetMachine].
    pub machine: TargetMachine,

    /// This field indicates the version fo the object file.
    pub version: u32,

    /// This field represents the virtual address of the entrypoint function. If there is not entry
    /// this field is null. In this API, the field is none when there is no address.
    pub entry_address: Option<u64>,

    /// This field indicates the in-file offset for the program header tables. If there are no
    /// program headers, this value is zero.
    pub program_header_offset: u64,

    /// This field indicates the in-file offset for the section header tables. If there are no
    /// section headers, this value is zero.
    pub section_header_offset: u64,

    /// This field holds target-specific flags.
    pub flags: u32,

    /// This field indicates the size of the ELF file header.
    pub file_header_size: u16,

    /// This field indicates the size of a single program header. All program headers have the same
    /// size.
    pub program_header_size: u16,

    /// This field indicates the count of the program headers in the file. If there are no program
    /// header, this value is zero.
    pub program_header_count: u16,

    /// This field indicates the size of a single section header. All section headers have the same
    /// size.
    pub section_header_size: u16,

    /// This field indicates the count of the section headers in the file. If there are no section
    /// header, this value is zero.
    pub section_header_count: u16,

    /// This member holds the index of the string table index. If there is no string table, this
    /// value is equal to `SHN_UNDEF`.
    pub string_table_index: u16
}

macro_rules! read_address_or_offset {
    ($ident_field: ident, $slice_field: ident, $offset: expr) => {
        match $ident_field.class {
            ElfClass::Invalid => return Err(Error::InvalidClass),
            ElfClass::Class32 => $ident_field.endian.read::<u32>($slice_field, Some($offset)).unwrap() as u64,
            ElfClass::Class64 => $ident_field.endian.read::<u64>($slice_field, Some($offset)).unwrap()
        }
    }
}

impl FileHeader {

    /// This function parses the specified slice with the offset to a ELF header. Most parts of the
    /// conversion is done with validation. After a successful parsing, this function returns
    /// the header structure.
    ///
    /// Here is a list with all errors, which can occur while this operation:
    /// - [Error::InvalidClass] - The provided ELF file's class is not valid
    pub fn read(slice: &[u8], mut offset: usize) -> Result<FileHeader, Error> {
        const IDENT_SIZE: usize = mem::size_of::<ElfIdent>();

        // Read indication bytes of file header
        let ident: ElfIdent = unsafe {
            mem::transmute::<[u8; IDENT_SIZE], ElfIdent>(
                slice.get(offset..(offset + IDENT_SIZE)).unwrap().try_into().unwrap()
            )
        };
        offset += 12;

        // Read platform-independent sized fields
        let ty = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();
        let machine = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();
        let version = ident.endian.read::<u32>(slice, Some(&mut offset)).unwrap();

        // Read entrypoint address and some offsets. We also read he size of this header.
        let entry_address = read_address_or_offset!(ident, slice, &mut offset);
        let program_header_offset = read_address_or_offset!(ident, slice, &mut offset);
        let section_header_offset = read_address_or_offset!(ident, slice, &mut offset);

        // Read size of this header and flags
        let flags = ident.endian.read::<u32>(slice, Some(&mut offset)).unwrap();
        let file_header_size = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();

        // Read count and size of program headers
        let program_header_size = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();
        let program_header_count = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();

        // Read count and size of section headers
        let section_header_size = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();
        let section_header_count = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();

        // Read index of string table header
        let string_table_index = ident.endian.read::<u16>(slice, Some(&mut offset)).unwrap();

        // Create file header and return
        Ok(Self {
            ident,
            ty: FileType::from(ty),
            machine: TargetMachine::from(machine),
            version,
            entry_address: if entry_address == 0 { None } else { Some(entry_address) },
            program_header_offset,
            section_header_offset,
            flags,
            file_header_size,
            program_header_size,
            program_header_count,
            section_header_size,
            section_header_count,
            string_table_index,
        })
    }
}