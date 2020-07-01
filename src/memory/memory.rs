use x86_64::structures::paging::PageTable;
use crate::utils::virtaddr::CanVirtAddr;

pub fn mut_table_l4(phys_offset: CanVirtAddr) -> &'static mut PageTable {
    let (l4_frame, _) = x86_64::registers::control::Cr3::read();
    let pt_ptr = (phys_offset.as_u64() +
                  l4_frame.start_address().as_u64()) as *mut PageTable;

    unsafe { &mut *pt_ptr }
}
