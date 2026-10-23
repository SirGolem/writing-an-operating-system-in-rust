#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let text_buffer = 0xB8000 as *mut u8;

    for (index, &byte) in b"Hello, world!".iter().enumerate() {
        unsafe {
            *text_buffer.offset(index as isize * 2) = byte;
            *text_buffer.offset(index as isize * 2 + 1) = 0xF;
        }
    }

    loop {}
}
