use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    number::complete::be_u8,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::types::header::eident::{
    EIClass, EIData, EIVersion, ElfIdent, EIOSABI, ELF_MAGIC_SIGNATURE,
};

pub fn parse_elf_magic_number(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(ELF_MAGIC_SIGNATURE)(i)
}

pub fn parse_elf_class(i: &[u8]) -> IResult<&[u8], EIClass> {
    map(be_u8, EIClass::from)(i)
}

pub fn parse_elf_data(i: &[u8]) -> IResult<&[u8], EIData> {
    map(be_u8, EIData::from)(i)
}

pub fn parse_elf_version(i: &[u8]) -> IResult<&[u8], EIVersion> {
    map(be_u8, EIVersion::from)(i)
}

pub fn parse_elf_osabi(i: &[u8]) -> IResult<&[u8], EIOSABI> {
    map(be_u8, EIOSABI::from)(i)
}

pub fn parse_elf_ident(i: &[u8]) -> IResult<&[u8], ElfIdent> {
    map(
        preceded(
            parse_elf_magic_number,
            terminated(
                tuple((
                    parse_elf_class,
                    parse_elf_data,
                    parse_elf_version,
                    parse_elf_osabi,
                    be_u8,
                )),
                take(7_u8),
            ),
        ),
        |(class, data, version, osabi, abi_version)| ElfIdent {
            class,
            data,
            version,
            osabi,
            abi_version,
        },
    )(i)
}

#[test]
fn test_parse_elf_ident() {
    assert_eq!(
        parse_elf_ident(&[
            0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ]),
        Ok((
            &[][..],
            ElfIdent {
                class: EIClass::Bit64,
                data: EIData::Lsb,
                version: EIVersion::Current,
                osabi: EIOSABI::SysV,
                abi_version: 0x00
            }
        ))
    );
}
