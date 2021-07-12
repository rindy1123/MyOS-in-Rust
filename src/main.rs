#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_project::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_project::println;

entry_point!(kernel_main);
#[cfg(not(test))]
#[panic_handler]
/// This function is called on panic.
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_project::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_project::test_panic_handler(info)
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, World");

    os_project::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os_project::hlt_loop();
}
