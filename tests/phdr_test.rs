extern crate elf_parser;
use elf_parser::{Elf32, Elf64};

#[test]
fn elf64_phdr() {
    let bytes = include_bytes!("./data/elf64");
    let elf64 = Elf64::from_bytes(bytes).unwrap();
    let ehdr = elf64.ehdr();
    let phdr_iter = elf64.phdr_iter();
    assert_eq!(elf64.phdr_num(), ehdr.e_phnum);
    for (i, phdr) in phdr_iter.enumerate() {
        assert_eq!(elf64.phdr_nth(i as u16).unwrap(), phdr.unwrap());
    }
    let phdr0 = elf64.phdr_nth(0).unwrap();
    assert_eq!(phdr0.p_type, 0x6);
    assert_eq!(phdr0.p_flags, 0x4);
    assert_eq!(phdr0.p_offset, 0x40);
    assert_eq!(phdr0.p_vaddr, 0x400040);
    assert_eq!(phdr0.p_paddr, 0x400040);
    assert_eq!(phdr0.p_filesz, 0x2d8);
    assert_eq!(phdr0.p_memsz, 0x2d8);
    assert_eq!(phdr0.p_align, 0x8);

    let phdr3 = elf64.phdr_nth(3).unwrap();
    assert_eq!(phdr3.p_type, 0x1);
    assert_eq!(phdr3.p_flags, 0x5);
    assert_eq!(phdr3.p_offset, 0x1000);
    assert_eq!(phdr3.p_vaddr, 0x401000);
    assert_eq!(phdr3.p_paddr, 0x401000);
    assert_eq!(phdr3.p_filesz, 0x125);
    assert_eq!(phdr3.p_memsz, 0x125);
    assert_eq!(phdr3.p_align, 0x1000);
}

#[test]
fn elf32_phdr() {
    let bytes = include_bytes!("./data/elf32");
    let elf32 = Elf32::from_bytes(bytes).unwrap();
    let ehdr = elf32.ehdr();
    let phdr_iter = elf32.phdr_iter();
    assert_eq!(elf32.phdr_num(), ehdr.e_phnum);
    for (i, phdr) in phdr_iter.enumerate() {
        assert_eq!(elf32.phdr_nth(i as u16).unwrap(), phdr.unwrap());
    }
    let phdr0 = elf32.phdr_nth(0).unwrap();
    assert_eq!(phdr0.p_type, 0x6);
    assert_eq!(phdr0.p_flags, 0x4);
    assert_eq!(phdr0.p_offset, 0x34);
    assert_eq!(phdr0.p_vaddr, 0x34);
    assert_eq!(phdr0.p_paddr, 0x34);
    assert_eq!(phdr0.p_filesz, 0x120);
    assert_eq!(phdr0.p_memsz, 0x120);
    assert_eq!(phdr0.p_align, 0x4);

    let phdr4 = elf32.phdr_nth(4).unwrap();
    assert_eq!(phdr4.p_type, 0x2);
    assert_eq!(phdr4.p_flags, 0x6);
    assert_eq!(phdr4.p_offset, 0xee4);
    assert_eq!(phdr4.p_vaddr, 0x1ee4);
    assert_eq!(phdr4.p_paddr, 0x1ee4);
    assert_eq!(phdr4.p_filesz, 0xf8);
    assert_eq!(phdr4.p_memsz, 0xf8);
    assert_eq!(phdr4.p_align, 0x4);
}
