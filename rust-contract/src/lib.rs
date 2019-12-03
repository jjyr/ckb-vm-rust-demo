#![no_std]

#[no_mangle]
pub extern "C" fn contract_entry() -> isize {
    return 13 + 29;
}

#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
