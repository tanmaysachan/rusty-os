// writes to the hardware VGA buffer

use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)] // enable copy-semantics
pub enum Col {
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

#[repr(transparent)]
struct StdBuffer {
    buf: [[Volatile<VgaPrintableChar>; BUF_WIDTH]; BUF_HEIGHT],
}

struct VgaWriter {
    col: usize,
    row: usize,
    vga_buf: &'static mut StdBuffer,
}

impl VgaWriter {
    pub fn write_char(&mut self, char_value: u8, color_code: u8) { // rust char is 4 bytes, need u8
        if self.row >= BUF_HEIGHT {
            self.new_line();
        }
        if char_value == b'\n' {
            self.row += 1;
        }
        else {
            self.vga_buf.buf[self.row][self.col].write(VgaPrintableChar {char_value, color_code});
            self.col += 1;
            if self.col >= BUF_WIDTH {
                self.col %= BUF_WIDTH;
                self.row += 1;
            }
        }
    }

    // reserving extra space for color_code for referring to default
    pub fn write_str(&mut self, ascii_string: &str, color_code: u8) {
        for char_value in ascii_string.bytes() {
            match char_value {
                0x20..=0x7e | b'\n' => self.write_char(char_value, color_code),
                // default to black square
                _ => self.write_char(0xfe, get_col(Col::Black, Col::Black)),
            }
        }
    }

    pub fn new_line(&mut self) {
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
        self.write_str(ascii_string, DEF_COLOR);
        Ok(())
    }
}


pub fn print_something() {
    use core::fmt::Write;
    let mut writer = VgaWriter {
        col: 0,
        row: 0,
        vga_buf: unsafe { &mut *(0xb8000 as *mut StdBuffer) },
    };

    writer.write_char(b'H', DEF_COLOR);
    writer.write_str("ello! ", DEF_COLOR);
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}
