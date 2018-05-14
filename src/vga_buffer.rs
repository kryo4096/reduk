use core::fmt::{self, Write};
use core::mem;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::port::Port;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
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

impl From<u8> for Color {
    fn from(byte: u8) -> Color {
        let byte = byte & 0b00001111;
        unsafe { mem::transmute_copy(&byte) }
    }
}

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const PORT_A: u16 = 0x3D4;
const PORT_B: u16 = 0x3D5;

type CharBuf = [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

struct VGABuffer {
    chars: &'static mut CharBuf,
}

impl VGABuffer {
    unsafe fn new() -> Self {
        let chars = &mut *(0xb8000 as *mut CharBuf);
        let mut port_a = Port::new(PORT_A);
        let mut port_b = Port::new(PORT_B);

        port_a.write(0x0Au8);
        port_b.write(0x20u8);

        VGABuffer { chars }
    }
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: VGABuffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;

                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);

        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn set_background(&mut self, background: Color) {
        self.color_code.0 &= 0b00001111;
        self.color_code.0 |= (background as u8) << 4;
    }

    fn set_foreground(&mut self, foreground: Color) {
        self.color_code.0 &= 0b11110000;
        self.color_code.0 |= foreground as u8;
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Black, Color::White),
            buffer: unsafe { VGABuffer::new() },
        };
        writer.clear();
        writer
    });
}

macro_rules! kprint {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

macro_rules! kprintln {
    () => (kprint!("\n"));
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear() {
    WRITER.lock().clear();
}

pub fn clear_bg(bg: Color) {
    set_background(bg);
    clear();
}

pub fn set_background(bg: Color) {
    WRITER.lock().set_background(bg);
}

pub fn set_foreground(fg: Color) {
    WRITER.lock().set_foreground(fg);
}
