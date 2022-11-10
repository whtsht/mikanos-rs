#![no_std]

use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::ScopedProtocol;

#[repr(C)]
#[derive(Debug)]
pub enum PixelFormat {
    RGBResv8BitPerColor,
    BGRResv8BitPerColor,
}

#[repr(C)]
#[derive(Debug)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PixelColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct FrameBuferConfig {
    pub pixels_per_scan_line: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
    pub pixel_format: PixelFormat,
    pub frame_buffer: *mut u8,
}

impl FrameBuferConfig {
    pub fn new(gop: &mut ScopedProtocol<GraphicsOutput>) -> Self {
        let info = gop.current_mode_info();
        let pixel_format = {
            match info.pixel_format() {
                uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::RGBResv8BitPerColor,
                uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::BGRResv8BitPerColor,
                _ => panic!(),
            }
        };

        FrameBuferConfig {
            horizontal_resolution: info.resolution().0,
            vertical_resolution: info.resolution().1,
            pixels_per_scan_line: info.stride(),
            pixel_format,
            frame_buffer: gop.frame_buffer().as_mut_ptr(),
        }
    }
}

unsafe fn write_pixel_bgr8bit(config: &FrameBuferConfig, x: usize, y: usize, color: &PixelColor) {
    let pixel_positon = config.pixels_per_scan_line * y + x;

    let p = config.frame_buffer.add(4 * pixel_positon);
    *p.offset(0) = color.b;
    *p.offset(1) = color.g;
    *p.offset(2) = color.r;
}

unsafe fn write_pixel_rgb8bit(config: &FrameBuferConfig, x: usize, y: usize, color: &PixelColor) {
    let pixel_positon = config.pixels_per_scan_line * y + x;

    let p = config.frame_buffer.add(4 * pixel_positon);
    *p.offset(0) = color.r;
    *p.offset(1) = color.g;
    *p.offset(2) = color.b;
}

#[repr(C)]
pub struct PixelWriter {
    pub config: FrameBuferConfig,
    pub writer: unsafe fn(&FrameBuferConfig, usize, usize, &PixelColor),
}

impl PixelWriter {
    /// # Safety
    /// 0 < x < self.horizontal_resolution
    /// 0 < y < self.vertical_resolution
    pub unsafe fn write_pixel(&mut self, x: usize, y: usize, color: &PixelColor) {
        (self.writer)(&self.config, x, y, color);
    }

    pub fn new(config: FrameBuferConfig) -> Self {
        let writer = match config.pixel_format {
            PixelFormat::RGBResv8BitPerColor => write_pixel_rgb8bit,
            PixelFormat::BGRResv8BitPerColor => write_pixel_bgr8bit,
        };

        Self { config, writer }
    }
}
