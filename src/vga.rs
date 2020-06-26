// writes to the hardware VGA buffer

use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)] // enable copy-semantics
enum Col {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// returns color code
const fn get_col(c: Col, bg: Col) -> u8 {
    (bg as u8) << 4 | c as u8
}

// VGA console size
const BUF_WIDTH: usize = 80;
const BUF_HEIGHT: usize = 25;

const DEF_COLOR: u8 = get_col(Col::White, Col::Black);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VgaPrintableChar {
    char_value: u8,
    color_code: u8,
}

/*
 * Explanation for volatile usage:
 * Compiler doesn't know anything about the existence
 * of a vga buffer at 0xb8000, it can optimise it away
 * as dead code.
 */
#[repr(transparent)]
struct StdBuffer {
    buf: [[Volatile<VgaPrintableChar>; BUF_WIDTH]; BUF_HEIGHT],
}

pub struct VgaWriter {
    col: usize,
    row: usize,
    vga_buf: &'static mut StdBuffer,
}

impl VgaWriter {
    pub fn write_char(&mut self, char_value: u8, color_code: u8) { // rust char is 4 bytes, need u8
        if self.row >= BUF_HEIGHT {
            self.new_line();
            self.row = BUF_HEIGHT-1;
        }
        if char_value == b'\n' {
            self.row += 1;
            self.col = 0;
        }
        else {
            self.vga_buf.buf[self.row][self.col].write(VgaPrintableChar {char_value, color_code});
            self.col += 1;
            if self.col >= BUF_WIDTH {
                self.col = 0;
                self.row += 1;
            }
        }
    }

    // reserving extra space for color_code for referring to default.
    // make write_str accessible through core::fmt:Write trait only.
    fn _write_str(&mut self, ascii_string: &str, color_code: u8) {
        for char_value in ascii_string.bytes() {
            match char_value {
                0x20..=0x7e | b'\n' => self.write_char(char_value, color_code),
                // default to black square
                _ => self.write_char(0xfe, get_col(Col::Black, Col::Black)),
            }
        }
    }

    fn new_line(&mut self) {
        for i in 0..BUF_HEIGHT-1 {
            for j in 0..BUF_WIDTH {
                let chr = self.vga_buf.buf[i+1][j].read();
                self.vga_buf.buf[i][j].write(chr);
                if i == BUF_HEIGHT - 1 {
                    self.vga_buf.buf[i+1][j].write(VgaPrintableChar{
                        char_value: b' ',
                        color_code: DEF_COLOR,
                    });
                }
            }
        }
    }

    pub fn clear_buf(&mut self) {
        for i in 0..BUF_HEIGHT {
            for j in 0..BUF_WIDTH {
                self.vga_buf.buf[i][j].write(VgaPrintableChar {
                    char_value: b' ',
                    color_code: DEF_COLOR,
                });
            }
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, ascii_string: &str) -> fmt::Result {
        self._write_str(ascii_string, DEF_COLOR);
        Ok(())
    }
}

// static writer
lazy_static! {
    // acquire lock before writing
    pub static ref GLOBAL_VGA_WRITER: Mutex<VgaWriter> =
        Mutex::new(VgaWriter {
                col: 0, // initialise to 0, 0
                row: 0,
                vga_buf: unsafe { &mut *(0xb8000 as *mut StdBuffer) },
            }
        );
}

// static writer functions
// meant to be accessed through macros
pub fn _write(args: fmt::Arguments) {
    use core::fmt::Write;
    GLOBAL_VGA_WRITER.lock().write_fmt(args).unwrap();
}
