pub fn apply_mask(num: &mut u16, st: u8, en: u8, mask: u16) {
    let mut mask = mask;
    *num &= !((1 << en) - 1) ^ ((1 << st) - 1);
    mask <<= st;
    mask &= ((1 << en) - 1) ^ ((1 << st) - 1);
    *num |= mask;
}

pub fn turn_on_range(num: &mut u16, st: u8, en: u8) {
    *num |= (((1 << en) - 1) ^ ((1 << st) - 1));
}

pub fn turn_on(num: &mut u16, index: u8) {
    *num |= (1 << index);
}

pub fn turn_off(num: &mut u16, index: u8) {
    *num &= !(1 << index);
}

pub fn turn_toggle(num: &mut u16, index: u8) {
    *num ^= (1 << index);
}
