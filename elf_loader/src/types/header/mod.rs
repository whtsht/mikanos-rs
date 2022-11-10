pub mod eident;
pub mod emachine;
pub mod etype;

use self::eident::ElfIdent;
use crate::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// The ELF header defines whether to use 32-bit or 64-bit addresses. The header contains three fields that are affected by this setting and offset other fields that follow them. The ELF header is 52 or 64 bytes long for 32-bit and 64-bit binaries respectively.
pub struct ElfHeader64 {
    pub e_ident: ElfIdent,
    /// Identifies object file type
    pub e_type: Elf64Half,
    /// Specifies target instruction set architecture. Some examples are:
    pub e_machine: Elf64Half,
    /// Set to 1 for the original version of ELF.
    pub e_version: Elf64Word,
    /// This is the memory address of the entry point from where the process starts executing. If the file doesn't have an associated entry point, then this holds zero.
    pub e_entry: Elf64Addr,
    /// Points to the start of the program header table
    pub e_phoff: Elf64Off,
    /// Points to the start of the section header table.
    pub e_shoff: Elf64Off,
    /// Interpretation of this field depends on the target architecture.
    pub e_flags: Elf64Word,
    /// Contains the size of this header, normally 64 Bytes for 64-bit and 52 Bytes for 32-bit format.
    pub e_ehsize: Elf64Half,
    /// Contains the size of a program header table entry.
    pub e_phentsize: Elf64Half,
    /// Contains the number of entries in the program header table.
    pub e_phnum: Elf64Half,
    /// Contains the size of a section header table entry.
    pub e_shentsize: Elf64Half,
    /// Contains the number of entries in the section header table.
    pub e_shnum: Elf64Half,
    /// Contains index of the section header table entry that contains the section names.
    pub e_shstrndx: Elf64Half,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElfHeader32 {
    pub e_ident: ElfIdent,
    pub e_type: Elf32Half,
    pub e_machine: Elf32Half,
    pub e_version: Elf32Word,
    pub e_entry: Elf32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
    pub e_flags: Elf32Word,
    pub e_ehsize: Elf32Half,
    pub e_phentsize: Elf32Half,
    pub e_phnum: Elf32Half,
    pub e_shentsize: Elf32Half,
    pub e_shnum: Elf32Half,
    pub e_shstrndx: Elf32Half,
}
