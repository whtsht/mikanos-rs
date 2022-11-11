use crate::{PixelColor, PixelWriter};

pub const KFONT_A: [u8; 16] = [
    0b00000000, //
    0b00011000, //    **
    0b00011000, //    **
    0b00011000, //    **
    0b00011000, //    **
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b01111110, //  ******
    0b01000010, //  *    *
    0b01000010, //  *    *
    0b01000010, //  *    *
    0b11100111, // ***  ***
    0b00000000, //
    0b00000000, //
];

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
