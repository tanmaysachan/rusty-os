mod interrupt_descriptor_table;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: interrupt_descriptor_table::Idt = {
        let mut idt = interrupt_descriptor_table::Idt::new();
        idt
    };
}
