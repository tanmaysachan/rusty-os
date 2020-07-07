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
* Make sure you're on a nightly rust build.
* Install qemu.
* Install xrun and xbuild for cargo for cross compilation.
* `cargo xbuild` for building(cargo calls qemu as a subcommand).
* `cargo xrun` for running.
