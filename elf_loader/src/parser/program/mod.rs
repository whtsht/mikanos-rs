use alloc::vec::Vec;
use nom::multi::count;
use nom::{
    combinator::map,
    number::complete::{le_u32, le_u64},
    sequence::tuple,
    IResult,
};

use crate::types::program::{ProgramHeader64, SegmentType};

pub fn parse_program_header64(i: &[u8]) -> IResult<&[u8], ProgramHeader64> {
    map(
        tuple((
            le_u32, le_u32, le_u64, le_u64, le_u64, le_u64, le_u64, le_u64,
        )),
        |(p_type, p_flags, p_offset, p_vaddr, p_paddr, p_filesz, p_memsz, p_align)| {
            ProgramHeader64 {
                p_type: SegmentType::from(p_type),
                p_flags,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_align,
            }
        },
    )(i)
}

pub fn parse_program_header_table64(
    entries: usize,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<ProgramHeader64>> {
    move |i: &[u8]| count(parse_program_header64, entries)(i)
}
