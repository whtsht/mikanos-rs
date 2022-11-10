#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(default_alloc_error_handler)]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use uart_16550::SerialPort;

use common::{FrameBuferConfig, PixelColor, PixelWriter};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "efiapi" fn kernel_main(config: FrameBuferConfig) -> ! {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    writeln!(serial_port, "Mandarin Kernel").unwrap();

    let mut pixel_writer = PixelWriter::new(config);

    for x in 100..pixel_writer.config.horizontal_resolution {
        for y in 100..pixel_writer.config.vertical_resolution {
            unsafe {
                pixel_writer.write_pixel(x, y, &PixelColor::new(255, 255, 0));
            }
        }
    }

    loop {
        unsafe { asm!("hlt") }
    }
}
