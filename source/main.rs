#![no_main]
#![no_std]

mod video_graphics_array_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    print_line!("{}", information);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_line!("Hello, world!");
    loop {}
}
