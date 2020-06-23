#![no_std]
#![no_main]

use rusty_os;
use rusty_os::println;
use core::panic::PanicInfo;

#[no_mangle] // disable compiler name mangling for _start
pub extern "C" fn _start() -> ! {
    rusty_os::init();
    println!("hello world!");

    // task successfully failed
    rusty_os::rnd_test();

    loop {}
}

#[panic_handler]
fn panic_kern(err: &PanicInfo) -> ! {
    println!("{}", err);
    loop {}
}
