use core::fmt;

pub const EI_NIDENT: usize = 16;
pub const MAGIC_NUM: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileType(pub u16);
pub const ET_NONE: FileType = FileType(0);
pub const ET_REL: FileType = FileType(1);
pub const ET_EXEC: FileType = FileType(2);
pub const ET_DYN: FileType = FileType(3);
pub const ET_CORE: FileType = FileType(4);
pub const ET_LOPROC: FileType = FileType(0xff00);
pub const ET_HIPROC: FileType = FileType(0xffff);

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.0 {
            0 => "ET_NONE",
            1 => "ET_REL",
            2 => "ET_EXEC",
            3 => "ET_DYN",
            4 => "ET_CORE",
            0xff00 => "ET_LOPROC",
            0xffff => "ET_HIPROC",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Machine(pub u16);
pub const EM_NONE: Machine = Machine(0);
pub const EM_M32: Machine = Machine(1);
pub const EM_SPARC: Machine = Machine(2);
pub const EM_386: Machine = Machine(3);
pub const EM_68K: Machine = Machine(4);
pub const EM_88K: Machine = Machine(5);
pub const EM_486: Machine = Machine(6);
pub const EM_860: Machine = Machine(7);
pub const EM_MIPS: Machine = Machine(8);
pub const EM_MIPS_RS3_LE: Machine = Machine(10);
pub const EM_MIPS_RS4_BE: Machine = Machine(10);
pub const EM_PARISC: Machine = Machine(15);
pub const EM_SPARC32PLUS: Machine = Machine(18);
pub const EM_PPC: Machine = Machine(20);
pub const EM_PPC64: Machine = Machine(21);
pub const EM_SPU: Machine = Machine(23);
pub const EM_ARM: Machine = Machine(40);
pub const EM_SH: Machine = Machine(42);
pub const EM_SPARCV9: Machine = Machine(43);
pub const EM_H8_300: Machine = Machine(46);
pub const EM_IA_64: Machine = Machine(50);
pub const EM_X86_64: Machine = Machine(62);
pub const EM_S390: Machine = Machine(22);
pub const EM_CRIS: Machine = Machine(76);
pub const EM_M32R: Machine = Machine(88);
pub const EM_MN10300: Machine = Machine(89);
pub const EM_OPENRISC: Machine = Machine(92);
pub const EM_ARCOMPACT: Machine = Machine(93);
pub const EM_XTENSA: Machine = Machine(94);
pub const EM_BLACKFIN: Machine = Machine(106);
pub const EM_UNICORE: Machine = Machine(110);
pub const EM_ALTERA_NIOS2: Machine = Machine(113);
pub const EM_TI_C6000: Machine = Machine(140);
pub const EM_HEXAGON: Machine = Machine(164);
pub const EM_NDS32: Machine = Machine(167);
pub const EM_AARCH64: Machine = Machine(183);
pub const EM_TILEPRO: Machine = Machine(188);
pub const EM_MICROBLAZE: Machine = Machine(189);
pub const EM_TILEGX: Machine = Machine(191);
pub const EM_ARCV2: Machine = Machine(195);
pub const EM_RISCV: Machine = Machine(243);
pub const EM_BPF: Machine = Machine(247);
pub const EM_CSKY: Machine = Machine(252);
pub const EM_FRV: Machine = Machine(0x5441);
pub const EM_ALPHA: Machine = Machine(0x9026);
pub const EM_CYGNUS_M32R: Machine = Machine(0x9041);
pub const EM_S390_OLD: Machine = Machine(0xA390);
pub const EM_CYGNUS_MN10300: Machine = Machine(0xbeef);

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.0 {
            0 => "EM_NONE",
            1 => "EM_M32",
            2 => "EM_SPARC",
            3 => "EM_386",
            4 => "EM_68K",
            5 => "EM_88K",
            6 => "EM_486",
            7 => "EM_860",
            8 => "EM_MIPS",
            10 => "EM_MIPS_RS3",
            15 => "EM_PARISC",
            18 => "EM_SPARC32PLUS",
            20 => "EM_PPC",
            21 => "EM_PPC64",
            23 => "EM_SPU",
            40 => "EM_ARM",
            42 => "EM_SH",
            43 => "EM_SPARCV9",
            46 => "EM_H8_300",
            50 => "EM_IA_64",
            62 => "EM_X86_64",
            22 => "EM_S390",
            76 => "EM_CRIS",
            88 => "EM_M32R",
            89 => "EM_MN10300",
            92 => "EM_OPENRISC",
            93 => "EM_ARCOMPACT",
            94 => "EM_XTENSA",
            106 => "EM_BLACKFIN",
            110 => "EM_UNICORE",
            113 => "EM_ALTERA_NIOS2",
            140 => "EM_TI_C6000",
            164 => "EM_HEXAGON",
            167 => "EM_NDS32",
            183 => "EM_AARCH64",
            188 => "EM_TILEPRO",
            189 => "EM_MICROBLAZE",
            191 => "EM_TILEGX",
            195 => "EM_ARCV2",
            243 => "EM_RISCV",
            247 => "EM_BPF",
            252 => "EM_CSKY",
            0x5441 => "EM_FRV",
            0x9026 => "EM_ALPHA",
            0x9041 => "EM_CYGNUS_M32R",
            0xA390 => "EM_S390_OLD",
            0xbeef => "EM_CYGNUS_MN10300",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileVersion(pub u32);
pub const EV_NONE: FileVersion = FileVersion(0);
pub const EV_CURRENT: FileVersion = FileVersion(1);
pub const EV_NUM: FileVersion = FileVersion(2);

impl fmt::Debug for FileVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.0 {
            0 => "EV_NONE",
            1 => "EV_CURRENT",
            2 => "EV_NUM",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Class(pub u8);

pub const ELFCLASSNONE: Class = Class(0);
pub const ELFCLASS32: Class = Class(1);
pub const ELFCLASS64: Class = Class(2);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PType(pub u32);
pub const PT_NULL: PType = PType(0);
pub const PT_LOAD: PType = PType(1);
pub const PT_DYNAMIC: PType = PType(2);
pub const PT_INTERP: PType = PType(3);
pub const PT_NOTE: PType = PType(4);
pub const PT_SHLIB: PType = PType(5);
pub const PT_PHDR: PType = PType(6);
pub const PT_TLS: PType = PType(7);
pub const PT_LOOS: PType = PType(0x60000000);
pub const PT_HIOS: PType = PType(0x6fffffff);
pub const PT_LOPROC: PType = PType(0x70000000);
pub const PT_HIPROC: PType = PType(0x7fffffff);
pub const PT_GNU_EH_FRAME: PType = PType(0x6474e550);
pub const PT_GNU_PROPERTY: PType = PType(0x6474e553);
pub const PT_GNU_STACK: PType = PType(0x6474e551);

impl fmt::Debug for PType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.0 {
            0 => "PT_NULL",
            1 => "PT_LOAD",
            2 => "PT_DYNAMIC",
            3 => "PT_INTERP",
            4 => "PT_NOTE",
            5 => "PT_SHLIB",
            6 => "PT_PHDR",
            7 => "PT_TLS",
            0x60000000 => "PT_LOOS",
            0x6fffffff => "PT_HIOS",
            0x70000000 => "PT_LOPROC",
            0x7fffffff => "PT_HIPROC",
            0x6474e550 => "PT_GNU_EH_FRAME",
            0x6474e553 => "PT_GNU_PROPERTY",
            0x6474e551 => "PT_GNU_STACK",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PFlag(pub u32);

impl fmt::Debug for PFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.0 {
            0 => "",
            1 => "  X",
            2 => " W",
            3 => " WX",
            4 => "R",
            5 => "R X",
            6 => "RW",
            7 => "RWX",
            _ => "INVALID",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShType(pub u32);

pub const SHT_NULL: ShType = ShType(0);
pub const SHT_PROGBITS: ShType = ShType(1);
pub const SHT_SYMTAB: ShType = ShType(2);
pub const SHT_STRTAB: ShType = ShType(3);
pub const SHT_RELA: ShType = ShType(4);
pub const SHT_HASH: ShType = ShType(5);
pub const SHT_DYNAMIC: ShType = ShType(6);
pub const SHT_NOTE: ShType = ShType(7);
pub const SHT_NOBITS: ShType = ShType(8);
pub const SHT_REL: ShType = ShType(9);
pub const SHT_SHLIB: ShType = ShType(10);
pub const SHT_DYNSYM: ShType = ShType(11);
pub const SHT_NUM: ShType = ShType(12);
pub const SHT_LOPROC: ShType = ShType(0x70000000);
pub const SHT_HIPROC: ShType = ShType(0x7fffffff);
pub const SHT_LOUSER: ShType = ShType(0x80000000);
pub const SHT_HIUSER: ShType = ShType(0xffffffff);

impl fmt::Debug for ShType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.0 {
            0 => "SHT_NULL",
            1 => "SHT_PROGBITS",
            2 => "SHT_SYMTAB",
            3 => "SHT_STRTAB",
            4 => "SHT_RELA",
            5 => "SHT_HASH",
            6 => "SHT_DYNAMIC",
            7 => "SHT_NOTE",
            8 => "SHT_NOBITS",
            9 => "SHT_REL",
            10 => "SHT_SHLIB",
            11 => "SHT_DYNSYM",
            12 => "SHT_NUM",
            0x70000000 => "SHT_LOPROC",
            0x7fffffff => "SHT_HIPROC",
            0x80000000 => "SHT_LOUSER",
            0xffffffff => "SHT_HIUSER",
            _ => "UNKNOWN",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShFlag(pub u64);

pub const SHF_WRITE: ShFlag = ShFlag(0x1);
pub const SHF_ALLOC: ShFlag = ShFlag(0x2);
pub const SHF_EXECINSTR: ShFlag = ShFlag(0x4);
pub const SHF_RELA_LIVEPATCH: ShFlag = ShFlag(0x00100000);
pub const SHF_RO_AFTER_INIT: ShFlag = ShFlag(0x00200000);
pub const SHF_MASKPROC: ShFlag = ShFlag(0xf0000000);

impl fmt::Debug for ShFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let write = if self.0 & 0x1 != 0 { "SHF_WRITE " } else { "" };
        let alloc = if self.0 & 0x2 != 0 { "SHF_ALLOC " } else { "" };
        let execinstr = if self.0 & 0x4 != 0 {
            "SHF_EXECINSTR "
        } else {
            ""
        };
        let rela = if self.0 & 0x00100000 != 0 {
            "SHF_RELA_LIVEPATCH "
        } else {
            ""
        };
        let ro = if self.0 & 0x00200000 != 0 {
            "SHF_RO_AFTER_INIT "
        } else {
            ""
        };
        let maskproc = if self.0 & 0xf0000000 != 0 {
            "SHF_MASKPROC "
        } else {
            ""
        };
        f.write_fmt(format_args!(
            "{}{}{}{}{}{}",
            write, alloc, execinstr, rela, ro, maskproc
        ))
    }
}
