use crate::Elf;
use crate::header::{FileType, TargetMachine};
use crate::header::ident::{ElfClass, ElfEndian, ElfOsABI, ElfVersion};

#[test]
fn test_elf_header() {
    let elf = Elf::from_bytes(include_bytes!("hello-world")).unwrap();

    // Test Ident Bytes
    let ident = &elf.file_header().ident;
    assert_eq!(ident.abi, ElfOsABI::Unspecified);
    assert_eq!(ident.class, ElfClass::Class64);
    assert_eq!(ident.endian, ElfEndian::Little);
    assert_eq!(ident.version, ElfVersion::Current);
    assert_eq!(ident.abi_version, 0);

    // Test Header Fields
    let header = elf.file_header();
    assert_eq!(header.ty, FileType::SharedObject);
    assert_eq!(header.machine, TargetMachine::X86_64);
    assert_eq!(header.entry_address, Some(0x87A0));
    assert_eq!(header.version, 0x1);
    assert_eq!(header.program_header_offset, 64);
    assert_eq!(header.section_header_offset, 4586344);
    assert_eq!(header.flags, 0x0);
    assert_eq!(header.file_header_size, 64);

    // Program header parameters in header
    assert_eq!(header.program_header_size, 56);
    assert_eq!(header.program_header_count, 14);

    // Section header parameters in header
    assert_eq!(header.section_header_size, 64);
    assert_eq!(header.section_header_count, 42);

    // Index of string table
    assert_eq!(header.string_table_index, 41);
}
