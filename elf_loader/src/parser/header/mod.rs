pub mod eident;
pub mod etype;

use nom::combinator::map;
use nom::number::complete::le_u16;
use nom::number::complete::le_u32;
use nom::number::complete::le_u64;
use nom::{sequence::tuple, IResult};

use crate::types::header::ElfHeader64;

use self::eident::parse_elf_ident;
pub fn parse_elf_header64(i: &[u8]) -> IResult<&[u8], ElfHeader64> {
    map(
        tuple((
            parse_elf_ident,
            le_u16,
            le_u16,
            le_u32,
            le_u64,
            le_u64,
            le_u64,
            le_u32,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
        )),
        |(
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
        )| ElfHeader64 {
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
        },
    )(i)
}
