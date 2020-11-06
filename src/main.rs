#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    panic!("O no errors");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("{}", info);
    loop {}
}