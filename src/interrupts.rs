use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;
use crate::vga_buffer;
use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Cyan, vga_buffer::Color::Black);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Green, vga_buffer::Color::Black);
}

// #[test_case]
// pub fn test_breakpoint_exception() {
//     x86_64::instructions::interrupts::int3();
// }