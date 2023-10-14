use crate::Elf;
use crate::header::{FileType, SegmentFlags, SegmentType, TargetMachine};
use crate::header::ident::{ElfClass, ElfEndian, ElfOsABI, ElfVersion};

#[test]
fn test_file_header() {
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

#[test]
fn test_program_headers() {
    let elf = Elf::from_bytes(include_bytes!("hello-world")).unwrap();
    let program_headers = elf.program_headers.as_ref().unwrap();

    // Check first program header
    let program_header = program_headers.get(0).unwrap();
    assert_eq!(program_header.ty, SegmentType::Phdr);
    assert_eq!(program_header.virtual_address, 0x40);
    assert_eq!(program_header.physical_address, 0x40);
    assert_eq!(program_header.file_size, 0x310);
    assert_eq!(program_header.memory_size, 0x310);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);

    // Check second program header
    let program_header = program_headers.get(1).unwrap();
    assert_eq!(program_header.ty, SegmentType::Interp);
    assert_eq!(program_header.virtual_address, 0x350);
    assert_eq!(program_header.physical_address, 0x350);
    assert_eq!(program_header.file_size, 0x1C);
    assert_eq!(program_header.memory_size, 0x1C);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);

    // Check third program header
    let program_header = program_headers.get(2).unwrap();
    assert_eq!(program_header.ty, SegmentType::Load);
    assert_eq!(program_header.virtual_address, 0x0);
    assert_eq!(program_header.physical_address, 0x0);
    assert_eq!(program_header.file_size, 0x55C8);
    assert_eq!(program_header.memory_size, 0x55C8);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);

    // Check fourth program header
    let program_header = program_headers.get(3).unwrap();
    assert_eq!(program_header.ty, SegmentType::Load);
    assert_eq!(program_header.virtual_address, 0x6000);
    assert_eq!(program_header.physical_address, 0x6000);
    assert_eq!(program_header.file_size, 0x42231);
    assert_eq!(program_header.memory_size, 0x42231);
    assert_eq!(program_header.flags, SegmentFlags::READABLE | SegmentFlags::EXECUTABLE);

    // Check fiftieth program header
    let program_header = program_headers.get(4).unwrap();
    assert_eq!(program_header.ty, SegmentType::Load);
    assert_eq!(program_header.virtual_address, 0x49000);
    assert_eq!(program_header.physical_address, 0x49000);
    assert_eq!(program_header.file_size, 0xFCBC);
    assert_eq!(program_header.memory_size, 0xFCBC);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);

    // Check sixth program header
    let program_header = program_headers.get(5).unwrap();
    assert_eq!(program_header.ty, SegmentType::Load);
    assert_eq!(program_header.virtual_address, 0x5A0D8);
    assert_eq!(program_header.physical_address, 0x5A0D8);
    assert_eq!(program_header.file_size, 0x2F58);
    assert_eq!(program_header.memory_size, 0x3068);
    assert_eq!(program_header.flags, SegmentFlags::READABLE | SegmentFlags::WRITABLE);

    // Check seventh program header
    let program_header = program_headers.get(6).unwrap();
    assert_eq!(program_header.ty, SegmentType::Dynamic);
    assert_eq!(program_header.virtual_address, 0x5C6C8);
    assert_eq!(program_header.physical_address, 0x5C6C8);
    assert_eq!(program_header.file_size, 0x210);
    assert_eq!(program_header.memory_size, 0x210);
    assert_eq!(program_header.flags, SegmentFlags::READABLE | SegmentFlags::WRITABLE);

    // Check eighth program header
    let program_header = program_headers.get(7).unwrap();
    assert_eq!(program_header.ty, SegmentType::Note);
    assert_eq!(program_header.virtual_address, 0x370);
    assert_eq!(program_header.physical_address, 0x370);
    assert_eq!(program_header.file_size, 0x20);
    assert_eq!(program_header.memory_size, 0x20);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);

    // Check ninth program header
    let program_header = program_headers.get(8).unwrap();
    assert_eq!(program_header.ty, SegmentType::Note);
    assert_eq!(program_header.virtual_address, 0x390);
    assert_eq!(program_header.physical_address, 0x390);
    assert_eq!(program_header.file_size, 0x44);
    assert_eq!(program_header.memory_size, 0x44);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);

    // Check tenth program header
    let program_header = program_headers.get(9).unwrap();
    assert_eq!(program_header.ty, SegmentType::TLS);
    assert_eq!(program_header.virtual_address, 0x5A0D8);
    assert_eq!(program_header.physical_address, 0x5A0D8);
    assert_eq!(program_header.file_size, 0x28);
    assert_eq!(program_header.memory_size, 0x50);
    assert_eq!(program_header.flags, SegmentFlags::READABLE);
}