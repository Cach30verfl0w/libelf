use bitflags::bitflags;
use crate::std::mem;
use crate::Error;
use crate::header::ident::{ElfClass, ElfIdent};

pub mod ident;

macro_rules! read_class_dependent {
    ($ident_field: expr, $slice_field: ident, $offset: expr) => {
        match $ident_field.class {
            ElfClass::Invalid => return Err(Error::InvalidClass),
            ElfClass::Class32 => $ident_field.endian.read::<u32>($slice_field, Some($offset)).unwrap() as u64,
            ElfClass::Class64 => $ident_field.endian.read::<u64>($slice_field, Some($offset)).unwrap()
        }
    }
}

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
    Core         = 4,
    Unknown(u16) = 5
}

impl From<u16> for FileType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Relocatable,
            2 => Self::Executable,
            3 => Self::SharedObject,
            4 => Self::Core,
            value => Self::Unknown(value)
        }
    }
}

/// This enum represents the target architecture/machine of the ELF file. This can be none for an
/// unknown or invalid target or one of the valid entries like x86_64 or ARM.
///
/// - [TargetMachine::None]: Unknown or invalid target architecture
/// - [TargetMachine::X86_64]: x86_64 as target architecture
/// - [TargetMachine::ARM]: ARM/AArch32 as target architecture
/// - [TargetMachine::ARM64]: ARM64/AArch64 as target architecture
/// - [TargetMachine::RISCV]: RISC-V as target architecture
///
/// ## See also
/// - [ELF Header](https://www.sco.com/developers/gabi/latest/ch4.eheader.html) by SCO, Inc.
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
        let entry_address = read_class_dependent!(ident, slice, &mut offset);
        let program_header_offset = read_class_dependent!(ident, slice, &mut offset);
        let section_header_offset = read_class_dependent!(ident, slice, &mut offset);

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

/// This enum contains all allowed types for segments in ELF files. These types are parsed by the
/// [ProgramHeader::read] function.
///
/// ## See also
/// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
#[repr(u32)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum SegmentType {
    /// This type is unused. Other member values are undefined so we ignore that type in loading
    /// etc.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    #[default]
    Null = 0x0,

    /// This type defines a loadable segment. So you should map the bytes in the header into the
    /// memory on the address. If the memory size is larger than the file size, the extra bytes must
    /// be filled with 0's.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    Load = 0x1,

    /// This type defines a section that contains dynamic linking information.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    Dynamic = 0x2,

    /// This type defines the location and size of a null-terminated path name. This segment is only
    /// meaningful for executable files and shared objects. It should be only one section with that
    /// type in an ELF file.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    Interp = 0x3,

    /// The array element specifies the location and size of auxiliary information.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    Note = 0x4,

    /// This type is reserved but has unspecified semantics. Programs that contain an array element
    /// of this type do not conform to the ABI.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    ShLib = 0x5,

    /// This type defines a section that specifies the location and size of the program header table
    /// itself.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    Phdr = 0x6,

    /// This type defines the Thread-Local Storage Template. Implementations doesn't need to support
    /// this program table section.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    TLS = 0x7,

    GNUProperty = 0x6474E553,
    GNUEhFrame = 0x6474E550,
    GNUStack = 0x6474E551,
    GNURelro = 0x6474E552,
    Unknown(u32) = 0xFFFFFFFF
}

impl From<u32> for SegmentType {
    fn from(value: u32) -> Self {
        match value {
            0x00000000 => Self::Null,
            0x00000001 => Self::Load,
            0x00000002 => Self::Dynamic,
            0x00000003 => Self::Interp,
            0x00000004 => Self::Note,
            0x00000005 => Self::ShLib,
            0x00000006 => Self::Phdr,
            0x00000007 => Self::TLS,
            0x6474E553 => Self::GNUProperty,
            0x6474E550 => Self::GNUEhFrame,
            0x6474E551 => Self::GNUStack,
            0x6474E552 => Self::GNURelro,
            value => Self::Unknown(value)
        }
    }
}

bitflags! {
    /// This structure represents the flags of a segment header/section. A section header can define
    /// three bits for the access.
    #[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
    pub struct SegmentFlags: u32 {
        /// The content of the section is executable
        const EXECUTABLE = 0x1;

        /// The content of the section is writable
        const WRITABLE   = 0x2;

        /// The content of the section is readable
        const READABLE   = 0x4;
    }
}

/// This structure contains the program segment header in a ELF file. The header contains the type,
/// the flags, offset, virtual and physical address, file and memory size and alignment of the
/// program section.
///
/// ## See also
/// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct ProgramHeader {
    /// This field represents the type of the segment. For more information, see [SegmentType].
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub ty: SegmentType,

    /// This field represents the flags of the segment.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub flags: SegmentFlags,

    /// This field indicates the offset of the segment in the ELF data.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub offset: u64,

    /// This field indicates the virtual address of the first byte in the memory.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub virtual_address: u64,

    /// On systems for which physical addressing is relevant, this member is reserved for the
    /// segment's physical address.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub physical_address: u64,

    /// This field indicates the size of the segment in the ELF file.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub file_size: u64,

    /// This field indicates the size of the segment in the memory.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub memory_size: u64,

    /// This field indicates the alignment of the segment in the memory.
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub alignment: u64
}

impl ProgramHeader {
    /// This function reads the data from the section (with offset) and parses it into a
    /// [ProgramHeader] structure for the ELF file.
    ///
    /// Here is a list with all errors, which can occur while this operation:
    /// - [Error::InvalidClass] - The provided ELF file's class is not valid
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub fn read(ident: &ElfIdent, slice: &[u8], mut offset: usize) -> Result<Self, Error> {
        let endian = &ident.endian;
        let mut program_header = Self::default();
        program_header.ty = SegmentType::from(ident.endian.read::<u32>(slice, Some(&mut offset)).unwrap());

        // Read elf flags if 64-bit ELF
        if ident.class == ElfClass::Class64 {
            program_header.flags = SegmentFlags::from_bits_retain(
                endian.read(slice, Some(&mut offset)).unwrap()
            );
        }

        // Read values in center of header
        program_header.offset = read_class_dependent!(ident, slice, &mut offset);
        program_header.virtual_address = read_class_dependent!(ident, slice, &mut offset);
        program_header.physical_address = read_class_dependent!(ident, slice, &mut offset);
        program_header.file_size = read_class_dependent!(ident, slice, &mut offset);
        program_header.memory_size = read_class_dependent!(ident, slice, &mut offset);

        // Read elf flags if 32-bit ELF
        if ident.class == ElfClass::Class32 {
            program_header.flags = SegmentFlags::from_bits_retain(
                endian.read(slice, Some(&mut offset)).unwrap()
            );
        }

        // Read alignment and return program header
        program_header.alignment = read_class_dependent!(ident, slice, &mut offset);
        Ok(program_header)
    }
}

/// This enum represents every available type of an ELF section. This enum is used by the library
/// to make the API more user-friendly.
#[repr(u32)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum SectionType {
    #[default]
    Null = 0,
    ProgBits = 1,
    SymbolTable = 2,
    StringTable = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    NoBits = 8,
    Rel = 9,
    ShLib = 10,
    DynamicSymbol = 11,
    InitArray = 14,
    FiniArray = 15,
    PreInitArray = 16,
    Group = 17,
    SymbolTableIndex = 81,
    Unknown(u32)
}

impl From<u32> for SectionType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Null,
            1 => Self::ProgBits,
            2 => Self::SymbolTable,
            3 => Self::StringTable,
            4 => Self::Rela,
            5 => Self::Hash,
            6 => Self::Dynamic,
            7 => Self::Note,
            8 => Self::NoBits,
            9 => Self::Rel,
            10 => Self::ShLib,
            11 => Self::DynamicSymbol,
            14 => Self::InitArray,
            15 => Self::FiniArray,
            16 => Self::PreInitArray,
            17 => Self::Group,
            81 => Self::SymbolTableIndex,
            value => Self::Unknown(value)
        }
    }
}

bitflags! {
    /// This structure contains all flags for a section in an ELF file
    #[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
    pub struct SectionFlags: u64 {
        /// This section is writable during execution
        const WRITE            = 0x1;

        /// This section occupies memory during process execution
        const ALLOC            = 0x2;

        /// This section contains executable machine instructions
        const INSTRUCTIONS     = 0x4;

        /// The data in this section should be merged to avoid duplication
        const MERGE            = 0x10;

        /// This data section holds null-terminated strings
        const STRINGS          = 0x20;

        /// The `info` field of this header contains a section header table index
        const INFO_LINK        = 0x40;

        /// This flag adds special ordering requirements for link editors
        const LINK_ORDER       = 0x80;

        /// This section requires special OS-specific processing to avoid incorrect behavior
        const OS_NONCONFORMING = 0x100;

        /// This section is a member of a group
        const GROUP            = 0x200;

        /// This section holds the thread-local-storage
        const TLS              = 0x400;

        /// This section contains compressed data
        const COMPRESSED       = 0x800;
    }
}

/// This structure represents the header of an ELF section. This header contains some information
/// about the section in the ELF file.
///
/// ## See also
/// - [Sections](https://www.sco.com/developers/gabi/latest/ch4.sheader.html) by SCO, Inc.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct SectionHeader {
    /// This field indicates the index of the name in the string table.
    pub name: u32,

    /// This field indicates the type of this section.
    pub ty: SectionType,

    /// This field indicates the flags of this section.
    pub flags: SectionFlags,

    /// This field indicates the address of the first byte, if this section will appear in the
    /// memory.
    pub addr: u64,

    /// This field indicates the offset of the first byte of the section from the start of the
    /// ELF data.
    pub offset: u64,

    /// This field indicates the size of the section in bytes.
    pub size: u64,

    /// This field indicates a section header table link index. (Interpretation depends on section
    /// type)
    pub link: u32,

    /// This field holds extra information about this section. (Interpretation depends on section
    /// type)
    pub info: u32,

    /// This field indicates the alignment for this section.
    pub addr_align: u64,

    /// This field indicates the size of fixed-size entries. This value is zero if there are no
    /// entries. This value is used in sections like the symbol table.
    pub entry_size: u64
}

impl SectionHeader {
    /// This function reads the data from the section (with offset) and parses it into a
    /// [SectionHeader] structure for the ELF file.
    ///
    /// Here is a list with all errors, which can occur while this operation:
    /// - [Error::InvalidClass] - The provided ELF file's class is not valid
    ///
    /// ## See also
    /// - [Program Header](https://www.sco.com/developers/gabi/latest/ch5.pheader.html) by SCO, Inc.
    pub fn read(ident: &ElfIdent, slice: &[u8], mut offset: usize) -> Result<Self, Error> {
        let endian = &ident.endian;
        let mut program_header = Self::default();
        program_header.name = endian.read::<u32>(slice, Some(&mut offset)).unwrap();
        program_header.ty = SectionType::from(endian.read::<u32>(slice, Some(&mut offset)).unwrap());
        program_header.flags = SectionFlags::from_bits_retain(read_class_dependent!(ident, slice, &mut offset));
        program_header.addr = read_class_dependent!(ident, slice, &mut offset);
        program_header.offset = read_class_dependent!(ident, slice, &mut offset);
        program_header.size = read_class_dependent!(ident, slice, &mut offset);
        program_header.link = endian.read::<u32>(slice, Some(&mut offset)).unwrap();
        program_header.info = endian.read::<u32>(slice, Some(&mut offset)).unwrap();
        program_header.addr_align = read_class_dependent!(ident, slice, &mut offset);
        program_header.entry_size = read_class_dependent!(ident, slice, &mut offset);
        Ok(program_header)
    }
}