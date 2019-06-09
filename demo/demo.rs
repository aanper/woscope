#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop{}
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 5
}
