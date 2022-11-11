#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

mod allocator;
mod console;
mod frame;

extern crate alloc;

use console::Console;
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use frame::{FrameBuferConfig, PixelColor, PixelWriter};
use uart_16550::SerialPort;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };

    if let Some(location) = panic_info.location() {
        writeln!(
            serial_port,
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        )
        .unwrap();
    } else {
        writeln!(
            serial_port,
            "panic occurred but can't get location information..."
        )
        .unwrap();
    }

    loop {
        unsafe { asm!("hlt") }
    }
}

#[no_mangle]
pub extern "efiapi" fn kernel_main(config: FrameBuferConfig) -> ! {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    writeln!(serial_port, "Mandarin Kernel").unwrap();

    let mut pixel_writer = PixelWriter::new(config);
    pixel_writer.clear_screen(&PixelColor::new(0, 0, 0));

    let mut console = Console::new(
        pixel_writer,
        PixelColor::new(255, 255, 255),
        PixelColor::new(0, 0, 0),
    );

    console.put_string("Hello, world.\n");
    console.put_string("I'm MikanOS.\n");
    console.put_string("Nice to meet you\n");

    loop {
        unsafe { asm!("hlt") }
    }
}
