#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ree_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("\n{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ree_os::test_panic_handler(info)
}