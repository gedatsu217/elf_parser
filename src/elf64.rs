use crate::{
    types::{
        Class, FileType, FileVersion, Machine, PFlag, PType, ShFlag, ShType, EI_NIDENT, ELFCLASS64,
        MAGIC_NUM,
    },
    util, Error,
};
use core::{fmt, mem};

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

impl<'a> Elf64Ehdr {
    fn new(bytes: &'a [u8]) -> Self {
        let e_ident = bytes[0..16].try_into().unwrap();
        let e_type = util::bytes_to_u16(&bytes[16..18]);
        let e_machine = util::bytes_to_u16(&bytes[18..20]);
        let e_version = util::bytes_to_u32(&bytes[20..24]);
        let e_entry = util::bytes_to_u64(&bytes[24..32]);
        let e_phoff = util::bytes_to_u64(&bytes[32..40]);
        let e_shoff = util::bytes_to_u64(&bytes[40..48]);
        let e_flags = util::bytes_to_u32(&bytes[48..52]);
        let e_ehsize = util::bytes_to_u16(&bytes[52..54]);
        let e_phentsize = util::bytes_to_u16(&bytes[54..56]);
        let e_phnum = util::bytes_to_u16(&bytes[56..58]);
        let e_shentsize = util::bytes_to_u16(&bytes[58..60]);
        let e_shnum = util::bytes_to_u16(&bytes[60..62]);
        let e_shstrndx = util::bytes_to_u16(&bytes[62..64]);
        Elf64Ehdr {
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
        }
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

impl<'a> Elf64Phdr {
    fn new(bytes: &'a [u8], offset: u64) -> Self {
        let offset_bytes = &bytes[offset as usize..];
        let p_type = util::bytes_to_u32(&offset_bytes[0..4]);
        let p_flags = util::bytes_to_u32(&offset_bytes[4..8]);
        let p_offset = util::bytes_to_u64(&offset_bytes[8..16]);
        let p_vaddr = util::bytes_to_u64(&offset_bytes[16..24]);
        let p_paddr = util::bytes_to_u64(&offset_bytes[24..32]);
        let p_filesz = util::bytes_to_u64(&offset_bytes[32..40]);
        let p_memsz = util::bytes_to_u64(&offset_bytes[40..48]);
        let p_align = util::bytes_to_u64(&offset_bytes[48..56]);
        Elf64Phdr {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        }
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

impl<'a> Elf64Shdr {
    fn new(bytes: &'a [u8], offset: u64) -> Self {
        let offset_bytes = &bytes[offset as usize..];
        let sh_name = util::bytes_to_u32(&offset_bytes[0..4]);
        let sh_type = util::bytes_to_u32(&offset_bytes[4..8]);
        let sh_flags = util::bytes_to_u64(&offset_bytes[8..16]);
        let sh_addr = util::bytes_to_u64(&offset_bytes[16..24]);
        let sh_offset = util::bytes_to_u64(&offset_bytes[24..32]);
        let sh_size = util::bytes_to_u64(&offset_bytes[32..40]);
        let sh_link = util::bytes_to_u32(&offset_bytes[40..44]);
        let sh_info = util::bytes_to_u32(&offset_bytes[44..48]);
        let sh_addralign = util::bytes_to_u64(&offset_bytes[48..56]);
        let sh_entsize = util::bytes_to_u64(&offset_bytes[56..64]);

        Elf64Shdr {
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
        }
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
            self.sh_name,
            ShType(self.sh_type),
            ShFlag(self.sh_flags),
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

pub struct Elf64<'a> {
    bytes: &'a [u8],
    ehdr: Elf64Ehdr,
}

impl<'a> Elf64<'_> {
    /// Get a Result<Elf64 struct, Error> from bytes.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Elf64, Error> {
        let length = bytes.len();

        if !bytes.starts_with(&MAGIC_NUM) {
            return Err(Error::InvalidMagicNumber);
        }

        if length < mem::size_of::<Elf64Ehdr>() {
            return Err(Error::Corrupted);
        }

        if Class(bytes[4] as u8) != ELFCLASS64 {
            return Err(Error::InvalidClass);
        }

        let ehdr = Elf64Ehdr::new(bytes);

        if !phdr_shdr_check(&ehdr, length) {
            return Err(Error::Corrupted);
        }

        Ok(Elf64 { bytes, ehdr })
    }

    /// Get a ELF header reference.
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

    /// Get a Result<nth program header, Error>.
    pub fn phdr_nth(&self, index: u16) -> Result<Elf64Phdr, Error> {
        let e_phoff = self.ehdr.e_phoff;
        let e_phentsize = self.ehdr.e_phentsize;
        let e_phnum = self.ehdr.e_phnum;

        let offset = e_phoff + index as u64 * e_phentsize as u64;

        if index >= e_phnum {
            return Err(Error::InvalidIndex);
        }

        Ok(Elf64Phdr::new(self.bytes, offset))
    }

    /// Get a nth program header, but not check bounds.
    pub fn phdr_nth_uncheck(&self, index: u16) -> Elf64Phdr {
        let e_phoff = self.ehdr.e_phoff;
        let e_phentsize = self.ehdr.e_phentsize;

        let offset = e_phoff + index as u64 * e_phentsize as u64;

        Elf64Phdr::new(self.bytes, offset)
    }

    /// Get a program header iterator.
    pub fn phdr_iter(&self) -> Elf64PhdrIter {
        Elf64PhdrIter {
            index: 0,
            elf64: self,
        }
    }

    /// Get the number of section headers.
    pub fn shdr_num(&self) -> u16 {
        self.ehdr.e_shnum
    }

    /// Get a Result<nth section header, Error>.
    pub fn shdr_nth(&self, index: u16) -> Result<Elf64Shdr, Error> {
        let e_shoff = self.ehdr.e_shoff;
        let e_shentsize = self.ehdr.e_shentsize;
        let e_shnum = self.ehdr.e_shnum;

        let offset = e_shoff + index as u64 * e_shentsize as u64;

        if index >= e_shnum {
            return Err(Error::InvalidIndex);
        }

        Ok(Elf64Shdr::new(self.bytes, offset))
    }

    /// Get a nth section header, but not check bounds.
    pub fn shdr_nth_uncheck(&self, index: u16) -> Elf64Shdr {
        let e_shoff = self.ehdr.e_shoff;
        let e_shentsize = self.ehdr.e_shentsize;

        let offset = e_shoff + index as u64 * e_shentsize as u64;

        Elf64Shdr::new(self.bytes, offset)
    }

    /// Get a section header iterator.
    pub fn shdr_iter(&self) -> Elf64ShdrIter {
        Elf64ShdrIter {
            index: 0,
            elf64: self,
        }
    }
}

fn phdr_shdr_check<'a>(ehdr: &Elf64Ehdr, length: usize) -> bool {
    let phnum = ehdr.e_phnum;
    let phoff = ehdr.e_phoff;
    let phsize = ehdr.e_phentsize;
    let phdr_end = phoff + phsize as u64 * phnum as u64;
    if phdr_end > length as u64 {
        return false;
    }

    let shnum = ehdr.e_shnum;
    let shoff = ehdr.e_shoff;
    let shsize = ehdr.e_shentsize;
    let shdr_end = shoff + shsize as u64 * shnum as u64;
    if shdr_end > length as u64 {
        return false;
    }
    true
}

pub struct Elf64PhdrIter<'a> {
    index: u16,
    elf64: &'a Elf64<'a>,
}

impl Iterator for Elf64PhdrIter<'_> {
    type Item = Elf64Phdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf64.ehdr.e_phnum {
            return None;
        }

        let phdr = self.elf64.phdr_nth_uncheck(self.index);
        self.index += 1;
        Some(phdr)
    }
}

pub struct Elf64ShdrIter<'a> {
    index: u16,
    elf64: &'a Elf64<'a>,
}

impl Iterator for Elf64ShdrIter<'_> {
    type Item = Elf64Shdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.elf64.ehdr.e_shnum {
            return None;
        }

        let shdr = self.elf64.shdr_nth_uncheck(self.index);
        self.index += 1;
        Some(shdr)
    }
}
