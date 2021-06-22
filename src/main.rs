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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {}:{}", 1002, 881.23).unwrap();

    loop {}
}

mod vga_buffer;
