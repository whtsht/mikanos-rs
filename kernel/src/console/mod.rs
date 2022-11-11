#![allow(dead_code)]
use crate::{frame::PixelWriter, PixelColor};

use self::font::{write_ascii, write_string};

pub mod font;

pub struct Console {
    writer: PixelWriter,
    fg_color: PixelColor,
    gb_color: PixelColor,
    buffer: [[char; Console::COLUMNS + 1]; Console::ROWS],
    cursor_row: usize,
    cursor_column: usize,
}

impl Console {
    const ROWS: usize = 25;
    const COLUMNS: usize = 80;
    const NULL_COL: [char; Console::COLUMNS + 1] = ['\0'; Console::COLUMNS + 1];

    pub fn new(writer: PixelWriter, fg_color: PixelColor, gb_color: PixelColor) -> Self {
        Self {
            writer,
            fg_color,
            gb_color,
            buffer: [['\0'; Console::COLUMNS + 1]; Console::ROWS],
            cursor_row: 0,
            cursor_column: 0,
        }
    }

    pub fn new_line(&mut self) {
        self.cursor_column = 0;
        if self.cursor_row < Console::ROWS - 1 {
            self.cursor_row += 1;
        } else {
            self.writer.clear_screen(&self.gb_color);

            for row in 0..Console::ROWS - 1 {
                self.buffer.swap(row, row + 1);
                unsafe {
                    write_string(
                        &mut self.writer,
                        0,
                        row * 16,
                        &self.buffer[row],
                        &self.fg_color,
                    );
                }
            }
            self.buffer[Console::ROWS - 1].copy_from_slice(&Console::NULL_COL);
        }
    }

    pub fn put_string(&mut self, string: &str) {
        for c in string.chars() {
            match c {
                '\n' => self.new_line(),
                '\t' => todo!(),
                _ => unsafe {
                    write_ascii(
                        &mut self.writer,
                        self.cursor_column * 8,
                        self.cursor_row * 16,
                        c,
                        &self.fg_color,
                    );
                    self.buffer[self.cursor_row][self.cursor_column] = c;
                    self.cursor_column += 1;
                },
            }
        }
    }
}
