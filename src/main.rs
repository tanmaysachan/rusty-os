#![no_std] // can't link against libc
#![no_main] // don't use regular entry point chain (crt0, start)

use core::panic::PanicInfo;

mod vga;
mod macros;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    loop {}
}

#[panic_handler]
fn panic_kern(err: &PanicInfo) -> ! {
    println!("{}", err);
    loop {}
}
