// canonical virtual address
pub struct CanVirtAddr(u64);

impl CanVirtAddr {
    pub fn new(addr: u64) -> Self {
        let stat = addr & (1 << 47) != 0;
        let mask = 0xFFFF000000000000;
        if stat {
            return CanVirtAddr(addr | mask);
        } else {
            return CanVirtAddr(addr & !mask);
        }
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}
