#![no_std] // can't link against libc
#![no_main] // don't use regular entry point chain (crt0, start)

use core::panic::PanicInfo;

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_something();
    loop {}
}

#[panic_handler]
fn panic_kern(_info: &PanicInfo) -> ! {
    loop {}
}
