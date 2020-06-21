// macros

// macros copied from println! and print! implementations in rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_write(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
