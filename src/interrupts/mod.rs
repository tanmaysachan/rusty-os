mod interrupt_descriptor_table;
mod interrupt_handlers;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: interrupt_descriptor_table::Idt = {
        let mut idt = interrupt_descriptor_table::Idt::new();
        idt.set_handler_fn(0, interrupt_handlers::divide_by_zero_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
