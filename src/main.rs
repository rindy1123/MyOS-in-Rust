#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
/// This function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

mod vga_buffer;
