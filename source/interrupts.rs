use crate::{global_descriptor_table::DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX, halt_loop, print, print_line};
use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1, layouts::Us104Key};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
    instructions::port::Port,
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

pub const PROGRAMMABLE_INTERRUPT_CONTROLLER_OFFSETS: (u8, u8) = (32, 32 + 8);

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut interrupt_descriptor_table = InterruptDescriptorTable::new();
        interrupt_descriptor_table.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            interrupt_descriptor_table
                .double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX);
        }

        interrupt_descriptor_table[InterruptIndex::Keyboard.value()].set_handler_fn(keyboard_handler);
        interrupt_descriptor_table.page_fault.set_handler_fn(page_fault_handler);
        interrupt_descriptor_table[InterruptIndex::Timer.value()].set_handler_fn(timer_handler);
        interrupt_descriptor_table
    };
}

pub static PROGRAMMABLE_INTERRUPT_CONTROLLERS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(PROGRAMMABLE_INTERRUPT_CONTROLLER_OFFSETS.0, PROGRAMMABLE_INTERRUPT_CONTROLLER_OFFSETS.1)
});

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) -> () {
    print_line!("Exception: Breakpoint reached.");
    print_line!("{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception: Double fault.\n{stack_frame:#?}");
}

pub fn initialize() -> () {
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

#[repr(u8)]
enum InterruptIndex {
    Timer = PROGRAMMABLE_INTERRUPT_CONTROLLER_OFFSETS.0,
    Keyboard,
}

impl InterruptIndex {
    fn value(self) -> u8 {
        self as u8
    }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) -> () {
    #[rustfmt::skip]
    static KEYBOARD: Mutex<PS2Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(PS2Keyboard::new(
        ScancodeSet1::new(),
        Us104Key,
        HandleControl::Ignore,
    ));

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scan_code: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                | DecodedKey::RawKey(key) => print!("{key:?}"),
                | DecodedKey::Unicode(character) => print!("{character}"),
            }
        }
    }

    unsafe { PROGRAMMABLE_INTERRUPT_CONTROLLERS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.value()) };
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) -> () {
    print_line!("Exception: Page fault.");
    print_line!("Accessed Address: {:?}", Cr2::read());
    print_line!("Error Code: {:?}", error_code);
    print_line!("{stack_frame:#?}");
    halt_loop();
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) -> () {
    print!(".");
    unsafe { PROGRAMMABLE_INTERRUPT_CONTROLLERS.lock().notify_end_of_interrupt(InterruptIndex::Timer.value()) };
}

#[cfg(test)]
mod tests {
    use x86_64::instructions::interrupts::int3;

    #[test_case]
    fn breakpoint_exception() -> () {
        int3();
    }
}
