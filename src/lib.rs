#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![no_std]

mod utils;
mod interrupts;
mod memory;
pub mod macros;
pub mod vga; // TODO: somehow decrease visibility of VGA

use x86_64;

pub fn init() {
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
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
