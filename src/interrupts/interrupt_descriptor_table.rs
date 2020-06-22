// my implementation of an IDT

// https://wiki.osdev.org/Interrupt_Descriptor_Table
use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

use crate::utils::bitops;

#[derive(Debug, Clone, Copy)]
pub struct Options(u16); // 16 bit operation field

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Interrupt {
    handler_pointer_lower: u16,
    gdt_selector: SegmentSelector,
    options: Options,
    handler_pointer_middle: u16,
    handler_pointer_high: u32,
    reserved: u32,
}

pub struct idt([Interrupt; 16]);

impl Options{
    fn initialise() -> Self {
        let mut options: u16 = 0;
        bitops::turn_on_range(&mut options, 9, 12);
        bitops::turn_on(&mut options, 15); // set present
        bitops::turn_off(&mut options, 8); // disable interrupts
        Options(options)
    }

    fn new() -> Self {
        let mut options = Self::initialise();
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        bitops::turn_on(&mut self.0, 15);
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
