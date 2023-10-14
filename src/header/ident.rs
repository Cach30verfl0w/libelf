/// This enum represents the `ELF_CLASS` field of the ident bytes in the header. This can be none if
/// the class is invalid, `CLASS32` if this file is a 32-bit object or `CLASS64` if this file is a
/// 64-bit object.
///
/// - [ElfClass::Invalid]: Invalid class specified
/// - [ElfClass::Class32]: 32-bit ELF File
/// - [ElfClass::Class64]: 64-bit ELF file
#[repr(u8)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum ElfClass {
    #[default]
    Invalid = 0,
    Class32 = 1,
    Class64 = 2
}

/// This enum represents the `ELF_DATA` field of the ident bytes in the header. This can be none if
/// the endianness is invalid, [`ElfEndian::Little`] when this file is little-endian encoded
/// or [`ElfEndian::Big`] when this file is big-endian encoded.
///
/// - [ElfEndian::Invalid]: Invalid endian specified
/// - [ElfEndian::Little]: Little endian
/// - [ElfEndian::Big]: Big endian
#[repr(u8)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum ElfEndian {
    #[default]
    Invalid = 0,
    Little = 1,
    Big = 2
}

/// This enum represents the version of the ELF file. This can currently be the current version (1)
/// or an invalid version.
///
/// - [ElfVersion::Invalid]: Invalid ELF version specified
/// - [ElfVersion::Current]: Current ELF version
#[repr(u8)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum ElfVersion {
    Invalid = 0,
    #[default]
    Current = 1
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub enum ElfOsABI {
    /// No extensions or unspecified
    #[default]
    Unspecified = 0x00,

    /// Hewlett-Packard HP-UX
    HP_UX = 0x01,

    /// NetBSD
    NetBSD = 0x02,

    /// GNU/Linux
    GNU = 0x03,

    /// Sun Solaris
    Solaris = 0x06,

    /// AIX
    AIX = 0x07,

    /// IRIX
    Irix = 0x08,

    /// FreeBSD
    FreeBSD = 0x09,

    /// Compaq TRU64 UNIX
    Tru64 = 0x0A,

    /// Novell Modesto
    Modesto = 0x0B,

    /// Open BSD
    OpenBSD = 0x0C,

    /// Open VMX
    OpenVMS = 0x0D,

    /// Hawlett-Packard Non-Stop Kernel
    NSK = 0x0E,

    /// Amiga Research OS
    AROS = 0x0F,

    /// The FenixOS highly scalable multi-core OS
    FenixOS = 0x10,

    /// Nuxi CloudABI
    CloudABI = 0x11,

    /// Stratus Technologies OpenVOS
    OpenVOX = 0x12
}

/// This structure represents the 9 bytes of the ident bytes, which can be found in the
/// [super::ElfHeader]. These bytes indicate the class, endianness, version, OS ABI and some more
/// data of the ELF file.
#[repr(C)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ElfIdent {
    /// This byte indicates the class of the ELF file. More details can be found in the [ElfClass]
    /// enum.
    pub class: ElfClass,

    /// This byte indicates the endianness of the ELF file. More details can be found in the
    /// [ElfEndian] enum.
    pub endian: ElfEndian,

    /// This byte indicates the version of the ELF file. More details can be found in the
    /// [ElfVersion] enum.
    pub version: ElfVersion,

    /// This byte indicates the ABI extensions of the ELF file. More details can be found in the
    /// [ElfOsABI] enum.
    pub abi: ElfOsABI,

    /// This byte indicates the ABI extensions version of the ELF file.
    pub abi_version: u8
}
