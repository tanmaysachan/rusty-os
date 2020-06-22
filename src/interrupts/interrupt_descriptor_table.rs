// my implementation of an IDT

// https://wiki.osdev.org/Interrupt_Descriptor_Table
use x86_64::instructions::segmentation;
use x86_64::structures::gdt;
use x86_64::PrivilegeLevel;

use crate::utils::bitops;

#[derive(Debug, Clone, Copy)]
pub struct Options(u16); // 16 bit operation field

// Handler function must have a type
// convertible to a 64 bit pointer address
// with C calling conventions
pub type HandlerFn = extern "C" fn() -> !;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Interrupt {
    handler_pointer_lower: u16,
    gdt_selector: gdt::SegmentSelector,
    options: Options,
    handler_pointer_middle: u16,
    handler_pointer_high: u32,
    reserved: u32,
}

impl Interrupt {
    // declare a new interrupt
    fn new(gdt_s: gdt::SegmentSelector, handler_fn: HandlerFn) -> Self {
        let addr: u64 = handler_fn as u64;
        Interrupt {
            handler_pointer_lower: addr as u16, // lower 16 bits
            gdt_selector: gdt_s,
            options: Options::initialise(),
            handler_pointer_middle: (addr << 16) as u16, // middle 16 bits
            handler_pointer_high: (addr << 32) as u32, // higher 32 bits
            reserved: 0,
        }
    }

    fn new_empty_int() -> Self {
        Interrupt {
            handler_pointer_lower: 0,
            gdt_selector: gdt::SegmentSelector::new(0, PrivilegeLevel::Ring3),
            options: Options::initialise(),
            handler_pointer_middle: 0,
            handler_pointer_high: 0,
            reserved: 0,
        }
    }
}

impl Options{
    fn initialise() -> Self {
        let mut options: u16 = 0;
        bitops::turn_on_range(&mut options, 9, 12);
        bitops::turn_on(&mut options, 15); // set present
        bitops::turn_off(&mut options, 8); // disable interrupts
        Options(options)
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            bitops::turn_on(&mut self.0, 15);
        } else {
            bitops::turn_off(&mut self.0, 15);
        }
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        if disable {
            bitops::turn_off(&mut self.0, 8);
        } else {
            bitops::turn_on(&mut self.0, 8);
        }
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        bitops::apply_mask(&mut self.0, 13, 15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, ind: u16) -> &mut Self {
        bitops::apply_mask(&mut self.0, 0, 3, ind);
        self
    }
}

pub struct Idt([Interrupt; 16]);

impl Idt {
    pub fn new() -> Self {
        Idt([Interrupt::new_empty_int(); 16])
    }

    pub fn set_handler_fn(&mut self, entry_no: u8, handler_fn: HandlerFn) {
        if entry_no < 16 {
            // cs is the code segment
            self.0[entry_no as usize] = Interrupt::new(segmentation::cs(), handler_fn);
        }
        // ignore call if not valid
    }

    // interface to change interrupt options
    pub fn modify_int_options(&mut self, entry_no: u8) -> &mut Options {
        // TODO: dont know how to handle error properly,
        // can't restrict out of bound access, std::Option
        // can't be used
        &mut self.0[entry_no as usize].options
    }

    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        // after loading, the stack frame can be overwritten,
        // preventable by static lifetime of self
        unsafe { lidt(&ptr) };
    }
}
