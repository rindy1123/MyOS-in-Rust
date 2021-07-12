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
    use os_project::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello, World");

    os_project::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os_project::hlt_loop();
}
