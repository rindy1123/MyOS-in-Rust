#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
/// This function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World, {}", 12345);
    panic!("Something went wrong");

    loop {}
}

mod vga_buffer;
