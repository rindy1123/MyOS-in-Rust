#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_project::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_project::println;

extern crate alloc;
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
    use os_project::allocator;
    use os_project::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello, World");

    os_project::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os_project::hlt_loop();
}
