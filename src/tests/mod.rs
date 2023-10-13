use crate::Elf;
use crate::header::ident::{ElfClass, ElfEndian, ElfOsABI, ElfVersion};

#[test]
fn test_elf_ident() {
    let elf = Elf::from_bytes(include_bytes!("hello-world")).unwrap();

    let ident = elf.file_header().ident();
    assert_eq!(ident.abi, ElfOsABI::Unspecified);
    assert_eq!(ident.class, ElfClass::Class64);
    assert_eq!(ident.endian, ElfEndian::Little);
    assert_eq!(ident.version, ElfVersion::Current);
    assert_eq!(ident.abi_version, 0);
}