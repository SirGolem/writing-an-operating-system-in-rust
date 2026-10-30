use core::fmt::{Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
use x86_64::instructions::interrupts::without_interrupts;

lazy_static! {
    #[rustfmt::skip]
    pub static ref SERIAL: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default()).expect("Failed to initialize serial device")
    });
}

#[doc(hidden)]
pub fn _print(arguments: Arguments) -> () {
    without_interrupts(|| {
        SERIAL.lock().write_fmt(arguments).expect("Failed to write to serial device");
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($argument:tt)*) => ($crate::serial::_print(format_args!($($argument)*)));
}

#[macro_export]
macro_rules! serial_print_line {
    () => ($crate::serial_print!("\n"));
    ($($argument:tt)*) => ($crate::serial_print!("{}\n", format_args!($($argument)*)));
}
