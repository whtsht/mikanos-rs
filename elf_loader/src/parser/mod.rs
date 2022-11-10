pub mod header;
pub mod program;
pub mod section;

use crate::parser::header::parse_elf_header64;
use crate::parser::section::parse_section_header_table64;
use crate::types::header::ElfHeader64;
use crate::types::section::Section64;
use alloc::vec::Vec;
use nom::IResult;

use self::section::parse_str_table_entry;

#[derive(Debug)]
pub struct Elf64<'a> {
    pub header: ElfHeader64,
    pub sections: Vec<Section64<'a>>,
}

pub fn parse_elf64(i: &[u8]) -> IResult<&[u8], Elf64> {
    let (_, header) = parse_elf_header64(i)?;

    let (_, sht) =
        parse_section_header_table64(header.e_shnum as usize)(&i[header.e_shoff as usize..])?;

    let section_names_offset = sht[header.e_shstrndx as usize].sh_offset;
    let section_names = &i[section_names_offset as usize..];

    let sections = sht
        .into_iter()
        .map(|header| Section64 {
            name: parse_str_table_entry(&section_names[header.sh_name as usize..])
                .unwrap()
                .1,
            header,
        })
        .collect::<Vec<Section64>>();

    Ok((&[], Elf64 { header, sections }))
}
