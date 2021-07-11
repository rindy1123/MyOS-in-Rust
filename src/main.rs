#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_project::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_project::println;

#[cfg(not(test))]
#[panic_handler]
/// This function is called on panic.
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_project::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World, {}", 12345);

    os_project::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {
        use os_project::print;

        for _ in 1..10000 {}
        print!("-");
    }
}
