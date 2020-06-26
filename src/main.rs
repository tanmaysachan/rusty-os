#![no_std]
#![no_main]

use rusty_os;
use rusty_os::{ println, print };
use core::panic::PanicInfo;
use x86_64;

#[no_mangle] // disable compiler name mangling for _start
pub extern "C" fn _start() -> ! {
    rusty_os::init();
    println!("hello world!");

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
