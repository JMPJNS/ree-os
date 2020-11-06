#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ree_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::Color;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    ree_os::init();

    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    #[cfg(test)]
    test_main();

    println!(":D");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_color(Color::Red, Color::Black);
    println!("\n{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ree_os::test_panic_handler(info)
}