#![cfg_attr(not(feature = "std"), no_std)]

pub mod parser;
pub mod types;

extern crate alloc;

#[allow(unused)]
/* Type for a 16-bit quantity.  */
/// Type for a 16-bit quantity (in ELF32)
pub type Elf32Half = u16;
/// Type for a 16-bit quantity (in ELF64)
pub type Elf64Half = u16;

/* Types for signed and unsigned 32-bit quantities.  */
/// Type for an unsigned 32-bit quantity (in ELF32)
pub type Elf32Word = u32;
/// Type for an unsigned 32-bit quantity (in ELF64)
pub type Elf64Word = u32;
/// Type for a signed 32-bit quantity (in ELF32)
pub type Elf32Sword = i32;
/// Type for a signed 32-bit quantity (in ELF64)
pub type Elf64Sword = i32;

/* Types for signed and unsigned 64-bit quantities.  */
/// Type for an unsigned 64-bit quantity (in ELF32)
pub type Elf32Xword = u64;
/// Type for an unsigned 64-bit quantity (in ELF64)
pub type Elf64Xword = u64;
/// Type for a signed 64-bit quantity (in ELF32)
pub type Elf32Sxword = i64;
/// Type for a signed 64-bit quantity (in ELF64)
pub type Elf64Sxword = i64;

/* Type of addresses.  */
/// Type of an address (in ELF32)
pub type Elf32Addr = u32;
/// Type of an address (in ELF64)
pub type Elf64Addr = u64;

/* Type of file offsets.  */
/// Type of a file offsets (in ELF32)
pub type Elf32Off = u32;
/// Type of a file offsets (in ELF64)
pub type Elf64Off = u64;

/* Type for section indices, which are 16-bit quantities.  */
/// Type of a section indices (in ELF32)
pub type Elf32Section = u16;
/// Type of a file offsets (in ELF64)
pub type Elf64Section = u16;

/* Type for version symbol information.  */
/// Type of a version symbol information (in ELF32)
pub type Elf32Versym = Elf32Half;
/// Type of a version symbol information (in ELF64)
pub type Elf64Versym = Elf64Half;
