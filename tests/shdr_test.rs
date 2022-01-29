extern crate elf_parser;
use elf_parser::{Elf64, Elf32};

#[test]
fn elf64_shdr() {
    let bytes = include_bytes!("./data/elf64");
    let elf64 = Elf64::from_bytes(bytes).unwrap();
    let ehdr = elf64.ehdr();
    let shdr_iter = elf64.shdr_iter();
    assert_eq!(elf64.shdr_num(), ehdr.e_shnum);
    for (i, shdr) in shdr_iter.enumerate() {
        assert_eq!(elf64.shdr_nth(i as u16).unwrap(), shdr.unwrap());
    }
    let shdr1 = elf64.shdr_nth(1).unwrap();
    assert_eq!(shdr1.sh_name, 0x1b);
    assert_eq!(shdr1.sh_type, 0x1);
    assert_eq!(shdr1.sh_flags, 0x2);
    assert_eq!(shdr1.sh_addr, 0x400318);
    assert_eq!(shdr1.sh_offset, 0x318);
    assert_eq!(shdr1.sh_size, 0x1c);
    assert_eq!(shdr1.sh_link, 0x0);
    assert_eq!(shdr1.sh_info, 0x0);
    assert_eq!(shdr1.sh_addralign, 0x1);
    assert_eq!(shdr1.sh_entsize, 0x0);

    let shdr17 = elf64.shdr_nth(17).unwrap();
    assert_eq!(shdr17.sh_name, 0xc9);
    assert_eq!(shdr17.sh_type, 0xe);
    assert_eq!(shdr17.sh_flags, 0x3);
    assert_eq!(shdr17.sh_addr, 0x403e50);
    assert_eq!(shdr17.sh_offset, 0x2e50);
    assert_eq!(shdr17.sh_size, 0x8);
    assert_eq!(shdr17.sh_link, 0x0);
    assert_eq!(shdr17.sh_info, 0x0);
    assert_eq!(shdr17.sh_addralign, 0x8);
    assert_eq!(shdr17.sh_entsize, 0x8);
}

#[test]
fn elf32_shdr() {
    let bytes = include_bytes!("./data/elf32");
    let elf32 = Elf32::from_bytes(bytes).unwrap();
    let ehdr = elf32.ehdr();
    let shdr_iter = elf32.shdr_iter();
    assert_eq!(elf32.shdr_num(), ehdr.e_shnum);
    for (i, shdr) in shdr_iter.enumerate() {
        assert_eq!(elf32.shdr_nth(i as u16).unwrap(), shdr.unwrap());
    }

    let shdr10 = elf32.shdr_nth(10).unwrap();
    assert_eq!(shdr10.sh_name, 0x83);
    assert_eq!(shdr10.sh_type, 0x9);
    assert_eq!(shdr10.sh_flags, 0x42);
    assert_eq!(shdr10.sh_addr, 0x350);
    assert_eq!(shdr10.sh_offset, 0x350);
    assert_eq!(shdr10.sh_size, 0x8);
    assert_eq!(shdr10.sh_link, 0x5);
    assert_eq!(shdr10.sh_info, 0x16);
    assert_eq!(shdr10.sh_addralign, 0x4);
    assert_eq!(shdr10.sh_entsize, 0x8);
    
    let shdr11 = elf32.shdr_nth(11).unwrap();
    assert_eq!(shdr11.sh_name, 0x8c);
    assert_eq!(shdr11.sh_type, 0x1);
    assert_eq!(shdr11.sh_flags, 0x6);
    assert_eq!(shdr11.sh_addr, 0x358);
    assert_eq!(shdr11.sh_offset, 0x358);
    assert_eq!(shdr11.sh_size, 0x23);
    assert_eq!(shdr11.sh_link, 0x0);
    assert_eq!(shdr11.sh_info, 0x0);
    assert_eq!(shdr11.sh_addralign, 0x4);
    assert_eq!(shdr11.sh_entsize, 0x0);

    
}