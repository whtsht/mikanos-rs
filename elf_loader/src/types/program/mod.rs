use crate::*;

#[derive(Debug)]
pub struct ProgramHeader64 {
    pub p_type: SegmentType,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Xword,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

#[derive(Debug)]
pub struct ProgramHeader32 {
    pub p_type: SegmentType,
    pub p_offset: Elf32Off,
    pub p_vaddr: Elf32Addr,
    pub p_paddr: Elf32Addr,
    pub p_filesz: Elf32Word,
    pub p_memsz: Elf32Word,
    pub p_flags: Elf32Word,
    pub p_align: Elf32Word,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SegmentType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    ShLib = 5,
    Phdr = 6,
    TLS = 7,
    Num = 8,
    GNUEHFrame = 0x6474e550,
    GNUStack = 0x6474e551,
    GNURelRO = 0x6474e552,
    Unknown,
}

impl From<u32> for SegmentType {
    fn from(v: u32) -> SegmentType {
        match v {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interp,
            4 => SegmentType::Note,
            5 => SegmentType::ShLib,
            6 => SegmentType::Phdr,
            7 => SegmentType::TLS,
            8 => SegmentType::Num,
            0x6474e550 => SegmentType::GNUEHFrame,
            0x6474e551 => SegmentType::GNUStack,
            0x6474e552 => SegmentType::GNURelRO,
            _ => SegmentType::Unknown,
        }
    }
}
