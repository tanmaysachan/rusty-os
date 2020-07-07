use bootloader::bootinfo::{ MemoryMap, MemoryRegionType };
use x86_64::structures::paging::{ PhysFrame, Size4KiB, FrameAllocator };
use x86_64::PhysAddr;

pub struct FrameAllocatorNew {
    mmap: &'static MemoryMap,
    nframe: usize,
}

impl FrameAllocatorNew {
    pub fn init(mmap: &'static MemoryMap) -> Self {
        // can have multiple mutable references to a frame
        unsafe { FrameAllocatorNew { mmap, nframe: 0 } }
    }

    // implement with a vec after heap impl
    fn usable_frames_iter(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.mmap.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for FrameAllocatorNew {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.nframe += 1;
        self.usable_frames_iter().nth(self.nframe - 1)
    }
}
