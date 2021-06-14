use crate::serial_println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_isr);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_isr(stack_frame: InterruptStackFrame) {
    serial_println!("Breakpoint ISR: {:?}", stack_frame);
}

#[test_case]
pub fn test_irq_is_handled() {
    x86_64::instructions::interrupts::int3();
}
