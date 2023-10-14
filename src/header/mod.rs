use std::mem;
use crate::header::ident::ElfIdent;

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
    pub(crate) ident: ElfIdent,
    pub(crate) ty: FileType,
    pub(crate) machine: TargetMachine
}

impl FileHeader {

    pub fn read(slice: &[u8], offset: usize) -> FileHeader {
        // Read indication bytes of file header
        let ident: ElfIdent = unsafe {
            mem::transmute::<[u8; mem::size_of::<ElfIdent>()], ElfIdent>(slice
                .get(offset..(offset + mem::size_of::<ElfIdent>())).unwrap().try_into().unwrap())
        };

        let ty = ident.endian.read::<u16>(slice, Some(offset + 12)).unwrap();
        let machine = ident.endian.read::<u16>(slice, Some(offset + 14)).unwrap();

        Self {
            ident,
            ty: FileType::from(ty),
            machine: TargetMachine::from(machine)
        }
    }

    #[inline]
    pub const fn ident(&self) -> &ElfIdent {
        &self.ident
    }
}