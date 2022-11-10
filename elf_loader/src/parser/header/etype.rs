#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EType {
    /// Unknown
    None,
    /// Relocatable file
    Rel,
    /// Executable file
    Exec,
    /// Shared object
    Dyn,
    /// Core file
    Core,
    /// Reserved inclusive range
    /// Operating system specific
    Loos,
    Hios,
    /// Reserved inclusive range
    /// Processor specific
    Loproc,
    Hiproc,
    Unknown(u64),
}

impl From<u64> for EType {
    fn from(b: u64) -> Self {
        match b {
            0 => EType::None,
            1 => EType::Rel,
            2 => EType::Exec,
            3 => EType::Dyn,
            4 => EType::Core,
            0xfe00 => EType::Loos,
            0xfeff => EType::Hios,
            0xff00 => EType::Loproc,
            0xffff => EType::Hiproc,
            _ => EType::Unknown(b),
        }
    }
}
