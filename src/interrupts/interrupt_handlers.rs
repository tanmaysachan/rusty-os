use crate::println;
use crate::print;
use crate::interrupts::PICS;

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    ip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

// Handler macro for handling interrupt_handlers without ERR
#[macro_export]
macro_rules! Handler {
    ($name: tt) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                llvm_asm!(concat!(
                       "push rax;",
                       "push rcx;",
                       "push rdx;",
                       "push rsi;",
                       "push rdi;",
                       "push r8;",
                       "push r9;",
                       "push r10;",
                       "push r11;")
                      :
                      :
                      :
                      : "intel", "volatile");

                llvm_asm!(concat!(
                       "mov rdi, rsp;",
                       "add rdi, 72;",
                       "call ", $name, ";")
                      // concatting name till i figure out how macros work :(
                      :
                      :
                      : "rdi"
                      : "intel", "volatile");

                llvm_asm!(concat!(
                       "pop r11;",
                       "pop r10;",
                       "pop r8;",
                       "pop r9;",
                       "pop rdi;",
                       "pop rsi;",
                       "pop rdx;",
                       "pop rcx;",
                       "pop rax;")
                      :
                      :
                      :
                      : "intel", "volatile");

                llvm_asm!(concat!(
                       "iretq;")
                      :
                      :
                      :
                      : "intel", "volatile");
                // force unreachability
                core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

// Handler macro for handling interrupt_handlers with ERR
#[macro_export]
macro_rules! WErrHandler {
    ($name: tt) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                llvm_asm!(concat!(
                       "push rax;",
                       "push rcx;",
                       "push rdx;",
                       "push rsi;",
                       "push rdi;",
                       "push r8;",
                       "push r9;",
                       "push r10;",
                       "push r11;")
                      :
                      :
                      :
                      : "intel", "volatile");
                llvm_asm!(concat!(
                       "mov rsi, [rsp + 72];",
                       "mov rdi, rsp;",
                       "add rdi, 80;",
                       "sub rsp, 8;",
                       "call ", $name, ";",
                       "add rsp, 8")
                      // concatting name till i figure out how macros work :(
                      :
                      :
                      : "rdi", "rsi"
                      : "intel");
                llvm_asm!(concat!(
                       "pop r11;",
                       "pop r10;",
                       "pop r8;",
                       "pop r9;",
                       "pop rdi;",
                       "pop rsi;",
                       "pop rdx;",
                       "pop rcx;",
                       "pop rax;")
                      :
                      :
                      :
                      : "intel", "volatile");
                llvm_asm!(concat!(
                       "add rsp, 8;",
                       "iretq;")
                      :
                      :
                      :
                      : "intel", "volatile");
                // force unreachability
                core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

// TODO: failing if providing address with name mangling
// not entirely sure if really required though

/* CPU exceptions */

// division by zero
#[no_mangle]
extern "C" fn
__hfn_divide_by_zero(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// debug
#[no_mangle]
extern "C" fn
__hfn_debug(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: DEBUG at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// non maskable interrupt
#[no_mangle]
extern "C" fn
__hfn_nmi(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: NON MASKABLE INTERRUPT at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// breakpoint
#[no_mangle]
extern "C" fn
__hfn_breakpoint(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// overflow
#[no_mangle]
extern "C" fn
__hfn_overflow(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: OVERFLOW at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// bound range exceeded
#[no_mangle]
extern "C" fn
__hfn_bound_range_exceeded(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: BOUND RANGE EXCEEDED at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// invalid opcode
#[no_mangle]
extern "C" fn
__hfn_invalid_opcode(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// device not available
#[no_mangle]
extern "C" fn
__hfn_device_not_available(sframe: &ExceptionStackFrame) {
    println!("\nEXCEPTION: DEVICE NOT AVAILABLE at {:#x}\n{:#?}",
             sframe.ip, sframe);
}

// double fault
#[no_mangle]
extern "C" fn
__hfn_df
(sframe: &ExceptionStackFrame, ecode: u64) {
    println!("\nEXCEPTION: DOUBLE FAULT ecode: {:?}\n{:#?}",
             ecode, sframe);
}

// invalid tss
#[no_mangle]
extern "C" fn
__hfn_invalid_tss
(sframe: &ExceptionStackFrame, ecode: u64) {
    println!("\nEXCEPTION: INVALID TSS ecode: {:?}\n{:#?}",
             ecode, sframe);
}

// segment not present fault
#[no_mangle]
extern "C" fn
__hfn_segment_not_present
(sframe: &ExceptionStackFrame, ecode: u64) {
    println!("\nEXCEPTION: SEGMENT NOT PRESENT ecode: {:?}\n{:#?}",
             ecode, sframe);
}

// stack segment fault
#[no_mangle]
extern "C" fn
__hfn_ssf
(sframe: &ExceptionStackFrame, ecode: u64) {
    println!("\nEXCEPTION: STACK SEGMENT FAULT ecode: {:?}\n{:#?}",
             ecode, sframe);
}

// general protection fault
#[no_mangle]
extern "C" fn
__hfn_gpf
(sframe: &ExceptionStackFrame, ecode: u64) {
    println!("\nEXCEPTION: GENERAL PROTECTION FAULT ecode: {:?}\n{:#?}",
             ecode, sframe);
}

// page fault
#[no_mangle]
extern "C" fn
__hfn_pf
(sframe: &ExceptionStackFrame, ecode: u64) {
    println!("\nEXCEPTION: PAGE FAULT ecode: {:?}\n{:#?}",
             ecode, sframe);
}

/* HARDWARE INTERRUPTS */

// timer interrupt
#[no_mangle]
extern "C" fn __hfn_timer_int (sframe: &ExceptionStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(32);
    }
}

// keyboard interrupt
#[no_mangle]
extern "C" fn __hfn_keyboard_int (sframe: &ExceptionStackFrame) {

    // read from PS/2 port
    let mut port = ::x86_64::instructions::port::Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref KEYBOARD: spin::Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            spin::Mutex::new(Keyboard::new(layouts::Us104Key,
                                           ScancodeSet1,
                                           HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}
