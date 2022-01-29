use crate::{util, Error, types::{FileType, Machine, FileVersion, PType, PFlag, ShType, ShFlag, Class, EI_NIDENT, MAGIC_NUM, ELFCLASS64}};
use core::fmt;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// ELF header of ELF64.
pub struct Elf64Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Elf64Ehdr {
    fn new(bytes: &'static [u8]) -> Result<Self, Error> {
        let e_ident = bytes[0..16].try_into().unwrap();
        let e_type = util::bytes_to_u16(&bytes[16..18])?;
        let e_machine = util::bytes_to_u16(&bytes[18..20])?;
        let e_version = util::bytes_to_u32(&bytes[20..24])?;
        let e_entry = util::bytes_to_u64(&bytes[24..32])?;
        let e_phoff = util::bytes_to_u64(&bytes[32..40])?;
        let e_shoff = util::bytes_to_u64(&bytes[40..48])?;
        let e_flags = util::bytes_to_u32(&bytes[48..52])?;
        let e_ehsize = util::bytes_to_u16(&bytes[52..54])?;
        let e_phentsize = util::bytes_to_u16(&bytes[54..56])?;
        let e_phnum = util::bytes_to_u16(&bytes[56..58])?;
        let e_shentsize = util::bytes_to_u16(&bytes[58..60])?;
        let e_shnum = util::bytes_to_u16(&bytes[60..62])?;
        let e_shstrndx = util::bytes_to_u16(&bytes[62..64])?;
        Ok(Elf64Ehdr {
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
            e_shstrndx
        })
    }
}

impl fmt::Debug for Elf64Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "ELF64Header: 
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
                FileType(self.e_type), Machine(self.e_machine), FileVersion(self.e_version), self.e_entry, self.e_phoff, self.e_shoff, self.e_flags, self.e_ehsize, self.e_phentsize, self.e_phnum, self.e_shentsize, self.e_shnum, self.e_shstrndx
        ))
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// Program header of ELF64.
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Elf64Phdr {
    fn new(bytes: &'static [u8], offset: u64) -> Result<Self, Error> {
        let offset_bytes = &bytes[offset as usize..];
        let p_type = util::bytes_to_u32(&offset_bytes[0..4])?;
        let p_flags = util::bytes_to_u32(&offset_bytes[4..8])?;
        let p_offset = util::bytes_to_u64(&offset_bytes[8..16])?;
        let p_vaddr = util::bytes_to_u64(&offset_bytes[16..24])?;
        let p_paddr = util::bytes_to_u64(&offset_bytes[24..32])?;
        let p_filesz = util::bytes_to_u64(&offset_bytes[32..40])?;
        let p_memsz = util::bytes_to_u64(&offset_bytes[40..48])?;
        let p_align = util::bytes_to_u64(&offset_bytes[48..56])?;
        Ok(Elf64Phdr{
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        })
    }
}

impl fmt::Debug for Elf64Phdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "ELF64ProgramHeader: 
    type: {:?}
    flags: {:?}
    offset: {:#x}
    vaddr: {:#x}
    paddr: {:#x}
    filesz: {:#x}
    memsz: {:#x}
    align: {:#x}",
            PType(self.p_type), PFlag(self.p_flags), self.p_offset, self.p_vaddr, self.p_paddr, self.p_filesz, self.p_memsz, self.p_align
        ))
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// Section Header of ELF64.
pub struct Elf64Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Elf64Shdr {
    fn new(bytes: &'static [u8], offset: u64) -> Result<Self, Error> {
        let offset_bytes = &bytes[offset as usize..];
        let sh_name = util::bytes_to_u32(&offset_bytes[0..4])?;
        let sh_type = util::bytes_to_u32(&offset_bytes[4..8])?;
        let sh_flags = util::bytes_to_u64(&offset_bytes[8..16])?;
        let sh_addr = util::bytes_to_u64(&offset_bytes[16..24])?;
        let sh_offset = util::bytes_to_u64(&offset_bytes[24..32])?;
        let sh_size = util::bytes_to_u64(&offset_bytes[32..40])?;
        let sh_link = util::bytes_to_u32(&offset_bytes[40..44])?;
        let sh_info = util::bytes_to_u32(&offset_bytes[44..48])?;
        let sh_addralign = util::bytes_to_u64(&offset_bytes[48..56])?;
        let sh_entsize = util::bytes_to_u64(&offset_bytes[56..64])?;

        Ok(Elf64Shdr {
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

impl fmt::Debug for Elf64Shdr {
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
            self.sh_name, ShType(self.sh_type), ShFlag(self.sh_flags), self.sh_addr, self.sh_offset, self.sh_size, self.sh_link, self.sh_info, self.sh_addralign, self.sh_entsize
        ))
    }
}

pub struct Elf64 {
    bytes: &'static [u8],
    ehdr: Elf64Ehdr,
}


impl Elf64 {
    /// Get a Elf64 struct from bytes.
    pub fn from_bytes(bytes: &'static [u8]) -> Result<Elf64, Error> {
        if !bytes.starts_with(&MAGIC_NUM) {
            return Err(Error::InvalidMagicNumber);
        }

        if Class(bytes[4] as u8) != ELFCLASS64 {
            return Err(Error::InvalidClass);
        }

        let ehdr = match Elf64Ehdr::new(bytes) {
            Ok(elf64) => elf64,
            Err(error) => return Err(error),
        };

        Ok(Elf64{bytes, ehdr})
    }

    /// Get a ELF header.
    pub fn ehdr(&self) -> &Elf64Ehdr {
        &self.ehdr
    }

    /// Get a entry point.
    pub fn entry_point(&self) -> u64 {
        self.ehdr.e_entry
    }

    /// Get the number of program headers.
    pub fn phdr_num(&self) -> u16 {
        self.ehdr.e_phnum
    }

    /// Get a nth program header.
    pub fn phdr_nth(&self, index: u16) -> Result<Elf64Phdr, Error> {
        let e_phoff = self.ehdr.e_phoff;
        let e_phentsize = self.ehdr.e_phentsize;
        let e_phnum = self.ehdr.e_phnum;

        let offset = e_phoff + index as u64 * e_phentsize as u64;

        if index >= e_phnum {
            return Err(Error::InvalidIndex);
        }

        Elf64Phdr::new(self.bytes, offset)
    }

    /// Get a program header iterator.
    pub fn phdr_iter(&self) -> Elf64PhdrIter {
        Elf64PhdrIter{
            index: 0,
            elf64: self,
        }
    }

    /// Get the number of section headers.
    pub fn shdr_num(&self) -> u16 {
        self.ehdr.e_shnum
    }

    /// Get a nth section header.
    pub fn shdr_nth(&self, index: u16) -> Result<Elf64Shdr, Error> {
        let e_shoff = self.ehdr.e_shoff;
        let e_shentsize = self.ehdr.e_shentsize;
        let e_shnum = self.ehdr.e_shnum;

        let offset = e_shoff + index as u64 * e_shentsize as u64;

        if index >= e_shnum {
            return Err(Error::InvalidIndex);
        }

        Elf64Shdr::new(self.bytes, offset)
    }

    /// Get a section header iterator.
    pub fn shdr_iter(&self) -> Elf64ShdrIter {
        Elf64ShdrIter{
            index: 0,
            elf64: self,
        }
    }
}

pub struct Elf64PhdrIter<'a> {
    index: u16,
    elf64: &'a Elf64,
}

impl Iterator for Elf64PhdrIter<'_> {
    type Item = Result<Elf64Phdr, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf64.ehdr.e_phnum {
            return None;
        }

        let phdr = self.elf64.phdr_nth(self.index);
        self.index += 1;
        Some(phdr)
    }
}

pub struct Elf64ShdrIter<'a> {
    index: u16,
    elf64: &'a Elf64,
}

impl Iterator for Elf64ShdrIter<'_> {
    type Item = Result<Elf64Shdr, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf64.ehdr.e_shnum {
            return None;
        }

        let shdr = self.elf64.shdr_nth(self.index);
        self.index += 1;
        Some(shdr)
    }
}