#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo, ptr};

global_asm!(include_str!("start.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
	const UART0: *mut u8 = 0x09000000 as *mut u8;
	let out_str = b"AArch64 Bare Metal";
	for byte in out_str {
		unsafe {
			ptr::write_volatile(UART0, *byte);
		}
	}
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
	loop {}
}
