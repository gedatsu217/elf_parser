extern crate elf_parser;
use elf_parser::Elf64;
 
fn main() {
    let bytes = include_bytes!("../tests/data/elf64");
    let elf64 = Elf64::from_bytes(bytes).unwrap();
    let ehdr = elf64.ehdr();
    dbg!(ehdr);
     
    let phdr_iter = elf64.shdr_iter();
    for phdr_res in phdr_iter {
        let phdr = phdr_res.unwrap();        
        dbg!(phdr);
    }
}