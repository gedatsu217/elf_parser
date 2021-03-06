//! A minimal no_std ELF (32/64) parser.
//!
//! ## Example Usage
//! ```ignore
//! use elf_parser::Elf64;
//!
//! fn main() {
//!     let bytes = include_bytes!("path/to/elf_file");
//!     let elf64 = Elf64::from_bytes(bytes).unwrap();
//!     let ehdr = elf64.ehdr();
//!     dbg!(ehdr);
//!     
//!     let phdr_iter = elf64.phdr_iter();
//!     for phdr in phdr_iter {     
//!         dbg!(phdr);
//!     }
//! }

#![no_std]
use core::fmt;

pub mod elf32;
pub mod elf64;
pub mod types;
mod util;

pub use elf32::{Elf32, Elf32Ehdr, Elf32Phdr, Elf32Shdr};
pub use elf64::{Elf64, Elf64Ehdr, Elf64Phdr, Elf64Shdr};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Magic numbers is not [0x7f, b'E', b'L', b'F'].
    InvalidMagicNumber,
    /// Out-of-bounds access to a page/section header table.
    InvalidIndex,
    /// The ELF file is loadead as a ELF32 although it is ELF64, and vice versa.
    InvalidClass,
    /// Failed to get header info. Probably, the file includes invalid value.
    Corrupted,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Error::InvalidMagicNumber => "InvalidMagicNumber",
            Error::InvalidIndex => "InvalidIndex",
            Error::InvalidClass => "InvalidClass",
            Error::Corrupted => "Corrupted",
        };
        f.write_fmt(format_args!("{}", name))
    }
}
