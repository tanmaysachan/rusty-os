#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![no_std]

mod utils;
mod interrupts;
pub mod macros;
pub mod vga; // TODO: somehow decrease visibility of VGA

pub fn init() {
    interrupts::init_idt();
}

// random scratch test, for debugging,
// till i write actual tests for this,
// which i should
pub fn rnd_test() {
    unsafe {
        llvm_asm!(
          "ud2"
        )
    };
}
