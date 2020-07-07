use x86_64::structures::paging::{ PageTable, OffsetPageTable };
use x86_64::VirtAddr;

fn mut_table_active_l4(phys_offset: VirtAddr) -> &'static mut PageTable {
    let (l4_frame, _) = x86_64::registers::control::Cr3::read();
    let pt_ptr = (phys_offset.as_u64() +
                  l4_frame.start_address().as_u64()) as *mut PageTable;

    unsafe { &mut *pt_ptr }
}

pub fn init_map(phys_offset: VirtAddr) -> OffsetPageTable<'static> {
    // get mutable active level 4 page table and init an offset page table
    unsafe { OffsetPageTable::new(mut_table_active_l4(phys_offset), phys_offset) }
}
