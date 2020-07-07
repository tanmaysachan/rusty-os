#![no_std]
#![no_main]

use rusty_os;
use rusty_os::{
    print,
    println,
};
use core::panic::PanicInfo;
use x86_64;
use rusty_os::utils;
use rusty_os::memory;
use rusty_os::memory::frame_allocator;

#[no_mangle] // disable compiler name mangling for _start
pub extern "C" fn _start(boot: &'static ::bootloader::BootInfo) -> ! {
    rusty_os::init();

    let mut frame_allocator = unsafe {
        frame_allocator::FrameAllocatorNew::init(&boot.memory_map) };

    
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic_kern(err: &PanicInfo) -> ! {
    println!("{}", err);
    loop {
        x86_64::instructions::hlt();
    }
}
