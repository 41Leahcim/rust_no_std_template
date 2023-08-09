#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate libc;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const u8) -> i32 {
    0
}
