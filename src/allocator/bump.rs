use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

use super::{Locked, align_up};

pub struct BumpAllocator {
	heap_start: usize,
	heap_end: usize,
	next: AtomicUsize, // Use AtomicUsize for safe mutation
	allocations: AtomicUsize,
}

impl BumpAllocator {
	/// Creates a new empty bump allocator.
	pub const fn new() -> Self {
		BumpAllocator {
			heap_start: 0,
			heap_end: 0,
			next: AtomicUsize::new(0),
			allocations: AtomicUsize::new(0),
		}
	}

	/// Initializes the bump allocator with the given heap bounds.
	///
	/// This method is unsafe because the caller must ensure that the given
	/// memory range is unused. Also, this method must be called only once.
	pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
		self.heap_start = heap_start;
		self.heap_end = heap_start + heap_size;
		self.next.store(heap_start, Ordering::SeqCst);
	}
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let bump = self.lock(); // get a mutable reference

		let alloc_start =
			align_up(bump.next.load(Ordering::SeqCst), layout.align());
		let alloc_end = match alloc_start.checked_add(layout.size()) {
			Some(end) => end,
			None => return ptr::null_mut(),
		};

		if alloc_end > bump.heap_end {
			ptr::null_mut() // out of memory
		} else {
			bump.next.store(alloc_end, Ordering::SeqCst);
			bump.allocations.fetch_add(1, Ordering::SeqCst);
			alloc_start as *mut u8
		}
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		let bump = self.lock(); // get a mutable reference

		bump.allocations.fetch_sub(1, Ordering::SeqCst);
		if bump.allocations.load(Ordering::SeqCst) == 0 {
			bump.next.store(bump.heap_start, Ordering::SeqCst);
		}
	}
}
