use crate::*;

#[derive(Debug)]
pub struct SectionHeader64 {
    pub sh_name: Elf64Word,
    pub sh_type: Elf64Word,
    pub sh_flags: Elf64Xword,
    pub sh_addr: Elf64Addr,
    pub sh_offset: Elf64Off,
    pub sh_size: Elf64Xword,
    pub sh_link: Elf64Word,
    pub sh_info: Elf64Word,
    pub sh_addralign: Elf64Xword,
    pub sh_entsize: Elf64Xword,
}

#[derive(Debug)]
pub struct SectionHeader32 {
    pub sh_name: Elf32Word,
    pub sh_type: Elf32Word,
    pub sh_flags: Elf32Word,
    pub sh_addr: Elf32Addr,
    pub sh_offset: Elf32Off,
    pub sh_size: Elf32Word,
    pub sh_link: Elf32Word,
    pub sh_info: Elf32Word,
    pub sh_addralign: Elf32Word,
    pub sh_entsize: Elf32Word,
}

#[derive(Debug)]
pub struct Section64<'a> {
    pub name: &'a str,
    pub header: SectionHeader64,
}
