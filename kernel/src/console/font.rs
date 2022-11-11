use crate::frame::{PixelColor, PixelWriter};

const FONT: &[u8] = include_bytes!("./hankaku.bin");

/// # Safety
/// 0 < x < self.horizontal_resolution - 8
/// 0 < y < self.vertical_resolution -16
pub unsafe fn write_ascii(
    writer: &mut PixelWriter,
    x: usize,
    y: usize,
    c: char,
    color: &PixelColor,
) {
    let idx = c as usize;
    let font = &FONT[16 * idx..16 * (idx + 1)];

    for (dy, f) in font.iter().enumerate() {
        for dx in 0..8 {
            if (f << dx) & 0x80 != 0 {
                writer.write_pixel(x + dx, y + dy, color);
            }
        }
    }
}

/// # Safety
/// 0 < x < self.horizontal_resolution - 8 * s.len()
/// 0 < y < self.vertical_resolution -16
pub unsafe fn write_string(
    writer: &mut PixelWriter,
    x: usize,
    y: usize,
    s: &[char],
    color: &PixelColor,
) {
    for (offset, &c) in s.iter().enumerate() {
        write_ascii(writer, x + offset * 8, y, c, color);
    }
}
