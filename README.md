# elf_parser
A minimal no_std library for parsing ELF (32/64).
[Documentation](https://docs.rs/elf_parser/0.1.1/elf_parser/)

## Example Usage
```
use elf_parser::Elf64;
 
fn main() {
    let bytes = include_bytes!("path/to/elf_file");
    let elf64 = Elf64::from_bytes(bytes).unwrap();
    let ehdr = elf64.ehdr();
    dbg!(ehdr);
     
    let phdr_iter = elf64.phdr_iter();
    for phdr in phdr_iter {      
        dbg!(phdr);
    }
}
```