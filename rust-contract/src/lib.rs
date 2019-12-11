#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

mod simple_alloc;

use core::cell::UnsafeCell;

// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program
#[global_allocator]
static HEAP: simple_alloc::BumpPointerAlloc = simple_alloc::BumpPointerAlloc {
    head: UnsafeCell::new(0x2000_0100),
    end: 0x2000_0200,
};

#[alloc_error_handler]
fn oom_handler(_: core::alloc::Layout) -> ! {
    extern "C" { fn abort() -> !; }
    unsafe { abort() }
}

#[no_mangle]
pub extern "C" fn contract_entry() -> isize {
    vec![13, 29].into_iter().sum()
}

#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
