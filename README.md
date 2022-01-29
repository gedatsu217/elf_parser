# elf_parser
A minimal no_std ELF (32/64) parser.

## Example Usage
```
use elf_parser::Elf64;
 
fn main() {
    let bytes = include_bytes!("path/to/elf_file");
    let elf64 = Elf64::from_bytes(bytes).unwrap();
    let ehdr = elf64.ehdr();
    dbg!(ehdr);
     
    let phdr_iter = elf64.phdr_iter();
    for phdr_res in phdr_iter {
        let phdr = phdr_res.unwrap();        
        dbg!(phdr);
    }
}
```