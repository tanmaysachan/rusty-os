#![no_std] // can't link against libc
#![no_main] // don't use regular entry point chain (crt0, start)

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    let vga_buffer = 0xb8000 as *mut u8;
    static HELLO: &[u8] = b"Hello World!";

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {
        

    }
}

#[panic_handler]
fn panic_kern(_info: &PanicInfo) -> ! {
    loop {

    }
}
