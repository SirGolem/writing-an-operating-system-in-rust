use lazy_static::lazy_static;
use x86_64::{
    VirtAddr,
    instructions::tables::load_tss,
    registers::segmentation::{CS, Segment},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX: u16 = 0;

lazy_static! {
    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
        let mut global_descriptor_table = GlobalDescriptorTable::new();
        let code_selector = global_descriptor_table.append(Descriptor::kernel_code_segment());
        let task_state_segment_selector = global_descriptor_table.append(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
        let selectors = Selectors {
            code: code_selector,
            task_state_segment: task_state_segment_selector,
        };
        (global_descriptor_table, selectors)
    };
}

lazy_static! {
    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut task_state_segment = TaskStateSegment::new();
        task_state_segment.interrupt_stack_table[DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };
        task_state_segment
    };
}

pub fn initialize() -> () {
    GLOBAL_DESCRIPTOR_TABLE.0.load();

    unsafe {
        CS::set_reg(GLOBAL_DESCRIPTOR_TABLE.1.code);
        load_tss(GLOBAL_DESCRIPTOR_TABLE.1.task_state_segment);
    }
}

struct Selectors {
    code: SegmentSelector,
    task_state_segment: SegmentSelector,
}
