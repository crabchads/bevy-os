use core::{mem::MaybeUninit, ptr};

pub unsafe fn mmio_write(addr: usize, bytes: &[u8]) {
	let dst = ptr::with_exposed_provenance_mut(addr);

	for byte in bytes {
		unsafe {
			ptr::write_volatile(dst, *byte);
		}
	}
}

pub unsafe fn mmio_read(addr: usize, bytes: &mut [MaybeUninit<u8>]) {
	let mut src = ptr::with_exposed_provenance(addr);

	for byte in bytes {
		*byte = MaybeUninit::new(unsafe { ptr::read_volatile(src) });
		src = unsafe { src.add(1) };
	}
}
