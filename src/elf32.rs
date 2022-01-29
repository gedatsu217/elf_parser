use crate::{
    types::{
        Class, FileType, FileVersion, Machine, PFlag, PType, ShFlag, ShType, EI_NIDENT, ELFCLASS32,
        MAGIC_NUM,
    },
    util, Error,
};
use core::fmt;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// ELF header of ELF32.
pub struct Elf32Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl<'a> Elf32Ehdr {
    fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        let e_ident = bytes[0..16].try_into().unwrap();
        let e_type = util::bytes_to_u16(&bytes[16..18])?;
        let e_machine = util::bytes_to_u16(&bytes[18..20])?;
        let e_version = util::bytes_to_u32(&bytes[20..24])?;
        let e_entry = util::bytes_to_u32(&bytes[24..28])?;
        let e_phoff = util::bytes_to_u32(&bytes[28..32])?;
        let e_shoff = util::bytes_to_u32(&bytes[32..36])?;
        let e_flags = util::bytes_to_u32(&bytes[36..40])?;
        let e_ehsize = util::bytes_to_u16(&bytes[40..42])?;
        let e_phentsize = util::bytes_to_u16(&bytes[42..44])?;
        let e_phnum = util::bytes_to_u16(&bytes[44..46])?;
        let e_shentsize = util::bytes_to_u16(&bytes[46..48])?;
        let e_shnum = util::bytes_to_u16(&bytes[48..50])?;
        let e_shstrndx = util::bytes_to_u16(&bytes[50..52])?;
        Ok(Elf32Ehdr {
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        })
    }
}

impl fmt::Debug for Elf32Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "ELF32Header: 
    type: {:?}
    machine: {:?}
    version: {:?}
    entry: {:#x}
    phoff: {:#x}
    shoff: {:#x}
    flags: {:#x}
    ehsize: {:#x}
    phentsize: {:#x}
    phnum: {:#x}
    shentsize: {:#x}
    shnum: {:#x}
    shstrndx: {:#x}",
            FileType(self.e_type),
            Machine(self.e_machine),
            FileVersion(self.e_version),
            self.e_entry,
            self.e_phoff,
            self.e_shoff,
            self.e_flags,
            self.e_ehsize,
            self.e_phentsize,
            self.e_phnum,
            self.e_shentsize,
            self.e_shnum,
            self.e_shstrndx
        ))
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// Program header of ELF32.
pub struct Elf32Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

impl<'a> Elf32Phdr {
    fn new(bytes: &'a [u8], offset: u64) -> Result<Self, Error> {
        let offset_bytes = &bytes[offset as usize..];
        let p_type = util::bytes_to_u32(&offset_bytes[0..4])?;
        let p_offset = util::bytes_to_u32(&offset_bytes[4..8])?;
        let p_vaddr = util::bytes_to_u32(&offset_bytes[8..12])?;
        let p_paddr = util::bytes_to_u32(&offset_bytes[12..16])?;
        let p_filesz = util::bytes_to_u32(&offset_bytes[16..20])?;
        let p_memsz = util::bytes_to_u32(&offset_bytes[20..24])?;
        let p_flags = util::bytes_to_u32(&offset_bytes[24..28])?;
        let p_align = util::bytes_to_u32(&offset_bytes[28..32])?;
        Ok(Elf32Phdr {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
            p_align,
        })
    }
}

impl fmt::Debug for Elf32Phdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "ELF32ProgramHeader: 
    type: {:?}
    flags: {:?}
    offset: {:#x}
    vaddr: {:#x}
    paddr: {:#x}
    filesz: {:#x}
    memsz: {:#x}
    align: {:#x}",
            PType(self.p_type),
            PFlag(self.p_flags),
            self.p_offset,
            self.p_vaddr,
            self.p_paddr,
            self.p_filesz,
            self.p_memsz,
            self.p_align
        ))
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// Section header of ELF32.
pub struct Elf32Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

impl<'a> Elf32Shdr {
    fn new(bytes: &'a [u8], offset: u64) -> Result<Self, Error> {
        let offset_bytes = &bytes[offset as usize..];
        let sh_name = util::bytes_to_u32(&offset_bytes[0..4])?;
        let sh_type = util::bytes_to_u32(&offset_bytes[4..8])?;
        let sh_flags = util::bytes_to_u32(&offset_bytes[8..12])?;
        let sh_addr = util::bytes_to_u32(&offset_bytes[12..16])?;
        let sh_offset = util::bytes_to_u32(&offset_bytes[16..20])?;
        let sh_size = util::bytes_to_u32(&offset_bytes[20..24])?;
        let sh_link = util::bytes_to_u32(&offset_bytes[24..28])?;
        let sh_info = util::bytes_to_u32(&offset_bytes[28..32])?;
        let sh_addralign = util::bytes_to_u32(&offset_bytes[32..36])?;
        let sh_entsize = util::bytes_to_u32(&offset_bytes[36..40])?;

        Ok(Elf32Shdr {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        })
    }
}

impl fmt::Debug for Elf32Shdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "ELF64SectionHeader: 
    name: {:#x}
    type: {:?}
    flags: {:?}
    addr: {:#x}
    offset: {:#x}
    size: {:#x}
    link: {:#x}
    info: {:#x}
    addralign: {:#x}
    entsize: {:#x}",
            self.sh_name,
            ShType(self.sh_type),
            ShFlag(self.sh_flags as u64),
            self.sh_addr,
            self.sh_offset,
            self.sh_size,
            self.sh_link,
            self.sh_info,
            self.sh_addralign,
            self.sh_entsize
        ))
    }
}

pub struct Elf32<'a> {
    bytes: &'a [u8],
    ehdr: Elf32Ehdr,
}

impl<'a> Elf32<'_> {
    /// Get a Elf32 struct from bytes.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Elf32, Error> {
        if !bytes.starts_with(&MAGIC_NUM) {
            return Err(Error::InvalidMagicNumber);
        }

        if Class(bytes[4] as u8) != ELFCLASS32 {
            return Err(Error::InvalidClass);
        }

        let ehdr = match Elf32Ehdr::new(bytes) {
            Ok(elf32) => elf32,
            Err(error) => return Err(error),
        };

        Ok(Elf32 { bytes, ehdr })
    }

    /// Get a ELF header.
    pub fn ehdr(&self) -> &Elf32Ehdr {
        &self.ehdr
    }

    /// Get a entry point.
    pub fn entry_point(&self) -> u32 {
        self.ehdr.e_entry
    }

    /// Get the number of program headers.
    pub fn phdr_num(&self) -> u16 {
        self.ehdr.e_phnum
    }

    /// Get a nth program header.
    pub fn phdr_nth(&self, index: u16) -> Result<Elf32Phdr, Error> {
        let e_phoff = self.ehdr.e_phoff;
        let e_phentsize = self.ehdr.e_phentsize;
        let e_phnum = self.ehdr.e_phnum;

        let offset = e_phoff as u64 + index as u64 * e_phentsize as u64;

        if index >= e_phnum {
            return Err(Error::InvalidIndex);
        }

        Elf32Phdr::new(self.bytes, offset)
    }

    /// Get a program header iterator.
    pub fn phdr_iter(&self) -> Elf32PhdrIter {
        Elf32PhdrIter {
            index: 0,
            elf32: self,
        }
    }

    /// Get the number of section headers.
    pub fn shdr_num(&self) -> u16 {
        self.ehdr.e_shnum
    }

    /// Get a nth section header.
    pub fn shdr_nth(&self, index: u16) -> Result<Elf32Shdr, Error> {
        let e_shoff = self.ehdr.e_shoff;
        let e_shentsize = self.ehdr.e_shentsize;
        let e_shnum = self.ehdr.e_shnum;

        let offset = e_shoff as u64 + index as u64 * e_shentsize as u64;

        if index >= e_shnum {
            return Err(Error::InvalidIndex);
        }

        Elf32Shdr::new(self.bytes, offset)
    }

    /// Get a section header iterator.
    pub fn shdr_iter(&self) -> Elf32ShdrIter {
        Elf32ShdrIter {
            index: 0,
            elf32: self,
        }
    }
}

pub struct Elf32PhdrIter<'a> {
    index: u16,
    elf32: &'a Elf32<'a>,
}

impl Iterator for Elf32PhdrIter<'_> {
    type Item = Result<Elf32Phdr, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf32.ehdr.e_phnum {
            return None;
        }

        let phdr = self.elf32.phdr_nth(self.index);
        self.index += 1;
        Some(phdr)
    }
}

pub struct Elf32ShdrIter<'a> {
    index: u16,
    elf32: &'a Elf32<'a>,
}

impl Iterator for Elf32ShdrIter<'_> {
    type Item = Result<Elf32Shdr, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf32.ehdr.e_shnum {
            return None;
        }

        let shdr = self.elf32.shdr_nth(self.index);
        self.index += 1;
        Some(shdr)
    }
}
