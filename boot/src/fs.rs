#![allow(dead_code)]
use alloc::boxed::Box;
use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::media::file::{
    Directory, File, FileAttribute, FileInfo, FileMode, FileType, RegularFile,
};
use uefi::CStr16;
use uefi::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn open_root_dir(image: Handle, bs: &BootServices) -> Result<Directory> {
    let mut sfs = bs.get_image_file_system(image)?;
    sfs.open_volume()
}

pub fn create(dir: &mut Directory, filename: &CStr16) -> Result<FileType> {
    dir.open(filename, FileMode::CreateReadWrite, FileAttribute::empty())?
        .into_type()
}
pub fn open(dir: &mut Directory, filename: &CStr16) -> Result<FileType> {
    dir.open(filename, FileMode::Read, FileAttribute::empty())?
        .into_type()
}

pub fn create_dir(dir: &mut Directory, filename: &CStr16) -> Result<FileType> {
    dir.open(
        filename,
        FileMode::CreateReadWrite,
        FileAttribute::DIRECTORY,
    )?
    .into_type()
}

pub fn create_file(dir: &mut Directory, filename: &CStr16) -> Result<RegularFile> {
    match create(dir, filename)? {
        FileType::Regular(file) => Ok(file),
        FileType::Dir(_) => panic!("Not a regular file: {}", filename),
    }
}

pub fn open_file(dir: &mut Directory, filename: &CStr16) -> Result<RegularFile> {
    match open(dir, filename)? {
        FileType::Regular(file) => Ok(file),
        FileType::Dir(_) => panic!("Not a regular file: {}", filename),
    }
}

pub fn get_file_info(file: &mut impl File) -> Result<Box<FileInfo>> {
    file.get_boxed_info::<FileInfo>()
}

pub fn read_file_to_vec(file: &mut RegularFile) -> Result<Vec<u8>> {
    let size = get_file_info(file)?.file_size() as usize;
    let mut buf = vec![0; size];
    file.read(&mut buf)
        .map_err(|err| Error::new(err.status(), ()))?;
    Ok(buf)
}
