#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hallo");

    #[cfg(test)]
    test_main();

    panic!("o no errors");

    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    let prev_color = vga_buffer::WRITER.lock().get_color();
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Yellow, vga_buffer::Color::Black);
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    vga_buffer::WRITER.lock().set_color_code(prev_color);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("\n{}", info);
    loop {}
}