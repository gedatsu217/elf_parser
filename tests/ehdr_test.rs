extern crate elf_parser;
use elf_parser::{Elf64, Elf32};

#[test]
fn not_elf() {
    let bytes = include_bytes!("./data/not_elf");
    let elf64 = Elf64::from_bytes(bytes);
    assert!(elf64.is_err());
}

#[test]
fn not_elf32() {
    let bytes = include_bytes!("./data/elf64");
    let elf32 = Elf32::from_bytes(bytes);
    assert!(elf32.is_err());
}   

#[test]
fn not_elf64() {
    let bytes = include_bytes!("./data/elf32");
    let elf64 = Elf64::from_bytes(bytes);
    assert!(elf64.is_err());
}

#[test]
fn elf64_ehdr() {
    let bytes = include_bytes!("./data/elf64");
    let elf64 = Elf64::from_bytes(bytes).unwrap();
    let ehdr = elf64.ehdr();
    assert_eq!(ehdr.e_ident, [0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
    assert_eq!(ehdr.e_type, 0x2);
    assert_eq!(ehdr.e_machine, 0x3e);
    assert_eq!(ehdr.e_version, 0x1);
    assert_eq!(ehdr.e_entry, 0x401020);
    assert_eq!(ehdr.e_phoff, 0x40);
    assert_eq!(ehdr.e_shoff, 0x54b0);
    assert_eq!(ehdr.e_flags, 0x0);
    assert_eq!(ehdr.e_ehsize, 0x40);
    assert_eq!(ehdr.e_phentsize, 0x38);
    assert_eq!(ehdr.e_phnum, 0xd);
    assert_eq!(ehdr.e_shentsize, 0x40);
    assert_eq!(ehdr.e_shnum, 0x1d);
    assert_eq!(ehdr.e_shstrndx, 0x1c);
}

#[test]
fn elf32_ehdr() {
    let bytes = include_bytes!("./data/elf32");
    let elf32 = Elf32::from_bytes(bytes).unwrap();
    let ehdr = elf32.ehdr();
    assert_eq!(ehdr.e_ident, [0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
    assert_eq!(ehdr.e_type, 0x3);
    assert_eq!(ehdr.e_machine, 0x3);
    assert_eq!(ehdr.e_version, 0x1);
    assert_eq!(ehdr.e_entry, 0x3b0);
    assert_eq!(ehdr.e_phoff, 0x34);
    assert_eq!(ehdr.e_shoff, 0x1788);
    assert_eq!(ehdr.e_flags, 0x0);
    assert_eq!(ehdr.e_ehsize, 0x34);
    assert_eq!(ehdr.e_phentsize, 0x20);
    assert_eq!(ehdr.e_phnum, 0x9);
    assert_eq!(ehdr.e_shentsize, 0x28);
    assert_eq!(ehdr.e_shnum, 0x1d);
    assert_eq!(ehdr.e_shstrndx, 0x1c);
}