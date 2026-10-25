use core::fmt::{Arguments, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
        color_code: ColorCode::new(BackgroundColor::Black, ForegroundColor::White),
        column: 0,
    });
}

#[allow(dead_code)]
#[repr(u8)]
pub enum BackgroundColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
}

#[repr(transparent)]
struct Buffer {
    characters: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[allow(dead_code)]
#[repr(u8)]
pub enum ForegroundColor {
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
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(background_color: BackgroundColor, foreground_color: ForegroundColor) -> ColorCode {
        ColorCode((background_color as u8) << 4 | (foreground_color as u8))
    }
}

#[doc(hidden)]
pub fn _print(arguments: Arguments) -> () {
    WRITER.lock().write_fmt(arguments).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($argument:tt)*) => ($crate::video_graphics_array_buffer::_print(format_args!($($argument)*)));
}

#[macro_export]
macro_rules! print_line {
    () => ($crate::print!("\n"));
    ($($argument:tt)*) => ($crate::print!("{}\n", format_args!($($argument)*)));
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ScreenCharacter {
    character: u8,
    color_code: ColorCode,
}

pub struct Writer {
    buffer: &'static mut Buffer,
    color_code: ColorCode,
    column: usize,
}

impl Writer {
    pub fn clear_line(&mut self, line: usize) -> () {
        let blank_character = ScreenCharacter {
            character: b' ',
            color_code: ColorCode::new(BackgroundColor::Black, ForegroundColor::Black),
        };

        for column in 0..BUFFER_WIDTH {
            self.buffer.characters[line][column].write(blank_character);
        }
    }

    pub fn new_line(&mut self) -> () {
        for column in 0..BUFFER_WIDTH {
            for line in 1..BUFFER_HEIGHT {
                let character = self.buffer.characters[line][column].read();
                self.buffer.characters[line - 1][column].write(character);
            }
        }

        self.clear_line(BUFFER_HEIGHT - 1);
        self.column = 0;
    }

    pub fn write_byte(&mut self, byte: u8) -> () {
        match byte {
            | b'\n' => self.new_line(),
            | byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let color_code = self.color_code;
                let column = self.column;
                let line = BUFFER_HEIGHT - 1;

                self.buffer.characters[line][column].write(ScreenCharacter {
                    character: byte,
                    color_code,
                });
                self.column += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) -> () {
        for byte in string.bytes() {
            match byte {
                | 0x20..=0x7E | b'\n' => self.write_byte(byte),
                | _ => self.write_byte(0xFE),
            }
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, string: &str) -> Result {
        self.write_string(string);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::video_graphics_array_buffer::{BUFFER_HEIGHT, WRITER};

    #[test_case]
    fn print_line() -> () {
        print_line!("tests::print_line");
    }

    #[test_case]
    fn print_line_multiple() -> () {
        for _ in 0..256 {
            print_line!("tests::print_line_multiple");
        }
    }

    #[test_case]
    fn print_line_output() -> () {
        let string = "Single-line test string.";
        print_line!("{string}");

        for (index, character) in string.chars().enumerate() {
            let screen_character = WRITER.lock().buffer.characters[BUFFER_HEIGHT - 2][index].read();
            assert_eq!(char::from(screen_character.character), character);
        }
    }
}
