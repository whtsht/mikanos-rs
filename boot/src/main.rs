#![no_main]
#![no_std]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

mod fs;

use core::fmt::Write;

use elf_loader::parser;
use elf_loader::types::program::SegmentType;
use kernel::frame::FrameBuferConfig;
use uart_16550::SerialPort;
use uefi::prelude::*;

use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::{AllocateType, MemoryType};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn load_kernel(image: Handle, st: &mut SystemTable<Boot>) -> fs::Result<usize> {
    let mut root_dir = fs::open_root_dir(image, st.boot_services())?;
    let mut kernel_elf = fs::open_file(&mut root_dir, cstr16!("kernel.elf"))?;
    let buffer = fs::read_file_to_vec(&mut kernel_elf)?;
    let header = parser::header::parse_elf_header64(&buffer)
        .expect("failed to parse elf header")
        .1;
    let pht = parser::program::parse_program_header_table64(header.e_phnum as usize)(
        &buffer[header.e_phoff as usize..],
    )
    .expect("failed to parse elf program header table")
    .1;

    let mut first_addr = usize::MAX;
    let mut last_addr = 0;
    for ph in pht.iter() {
        if ph.p_type == SegmentType::Load {
            first_addr = first_addr.min(ph.p_vaddr as usize);
            last_addr = last_addr.max((ph.p_vaddr + ph.p_memsz) as usize);
        }
    }
    const ELF_PAGE_SIZE: usize = 1000;

    st.boot_services().allocate_pages(
        AllocateType::Address(first_addr),
        MemoryType::LOADER_DATA,
        (last_addr - first_addr + ELF_PAGE_SIZE - 1) / ELF_PAGE_SIZE,
    )?;

    for ph in pht.iter() {
        if ph.p_type != SegmentType::Load {
            continue;
        }
        let ofs = ph.p_offset as usize;
        let fsize = ph.p_filesz as usize;
        let msize = ph.p_memsz as usize;
        let dest = unsafe { core::slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, msize) };
        dest[..fsize].copy_from_slice(&buffer[ofs..ofs + fsize]);
        dest[fsize..].fill(0);
    }

    Ok(header.e_entry as usize)
}

fn run_kernel(entry_point_addr: usize, config: FrameBuferConfig) {
    let entry_point: extern "efiapi" fn(FrameBuferConfig) =
        unsafe { core::mem::transmute(entry_point_addr) };
    entry_point(config);
}

#[entry]
fn _start(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();

    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();

    writeln!(serial_port, "Mandarin Loader").unwrap();
    let config = {
        let graphic_output_handle = st
            .boot_services()
            .get_handle_for_protocol::<GraphicsOutput>()
            .unwrap();

        let mut gop = st
            .boot_services()
            .open_protocol_exclusive::<GraphicsOutput>(graphic_output_handle)
            .unwrap();
        FrameBuferConfig::new(&mut gop)
    };

    let entry_point_addr = load_kernel(handle, &mut st).unwrap();
    run_kernel(entry_point_addr, config);

    Status::SUCCESS
}
