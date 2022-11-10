use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map, map_res},
    number::complete::{le_u32, le_u64},
    sequence::{terminated, tuple},
    IResult,
};

use crate::types::section::SectionHeader64;

pub fn parse_section_header64(i: &[u8]) -> IResult<&[u8], SectionHeader64> {
    map(
        tuple((
            le_u32, le_u32, le_u64, le_u64, le_u64, le_u64, le_u32, le_u32, le_u64, le_u64,
        )),
        |(
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
        )| SectionHeader64 {
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
        },
    )(i)
}

pub fn parse_section_header_table64<'a>(
    entries: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], alloc::vec::Vec<SectionHeader64>> {
    move |i: &'a [u8]| nom::multi::count(parse_section_header64, entries)(i)
}

pub fn parse_ascii_str(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(take_while(|c: u8| c.is_ascii_graphic()), |r: &[u8]| {
        core::str::from_utf8(r)
    })(i)
}

pub fn parse_str_table_entry(i: &[u8]) -> IResult<&[u8], &str> {
    terminated(parse_ascii_str, tag(b"\x00"))(i)
}
