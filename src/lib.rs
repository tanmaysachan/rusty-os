#![feature(llvm_asm)]
#![no_std]
mod utils;
mod interrupts;
pub mod macros;
pub mod vga; // TODO: somehow decrease visibility of VGA

pub fn init() {
    interrupts::init_idt();
}

pub fn rnd_test() {
    unsafe {
        llvm_asm!(
            "mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel"
        )
    }
}
