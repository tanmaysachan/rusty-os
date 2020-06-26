mod interrupt_descriptor_table;
mod interrupt_handlers;
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;

lazy_static! {
    static ref IDT: interrupt_descriptor_table::Idt = {
        let mut idt = interrupt_descriptor_table::Idt::new();

        // https://wiki.osdev.org/Exceptions
        use crate::{ Handler, WErrHandler };
        idt.set_handler_fn(0, Handler!("__hfn_divide_by_zero"));
        idt.set_handler_fn(1, Handler!("__hfn_debug"));
        idt.set_handler_fn(2, Handler!("__hfn_nmi"));
        idt.set_handler_fn(3, Handler!("__hfn_breakpoint"));
        idt.set_handler_fn(4, Handler!("__hfn_overflow"));
        idt.set_handler_fn(5, Handler!("__hfn_bound_range_exceeded"));
        idt.set_handler_fn(6, Handler!("__hfn_invalid_opcode"));
        idt.set_handler_fn(7, Handler!("__hfn_device_not_available"));
        idt.set_handler_fn(8, WErrHandler!("__hfn_df"));
        idt.set_handler_fn(10, WErrHandler!("__hfn_invalid_tss"));
        idt.set_handler_fn(11, WErrHandler!("__hfn_segment_not_present"));
        idt.set_handler_fn(12, WErrHandler!("__hfn_ssf"));
        idt.set_handler_fn(13, WErrHandler!("__hfn_gpf"));
        idt.set_handler_fn(14, WErrHandler!("__hfn_pf"));

        idt.set_handler_fn(32, Handler!("__hfn_timer_interrupt"));

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
