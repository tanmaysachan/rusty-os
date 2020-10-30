### Stuff done:
* bootable rust binary
* vga writer
* an IDT
* Fault handling
* paging
* frame allocations

### Stuff left:
(Everything else)
Notably,
* heap allocations
* processes

### Steps to build:
* Make sure you're on atleast 2020-09-17 nightly rust build.
* Install qemu.
* `cargo install bootimage`
* `cargo run`
