#![no_main]
#![no_std]

use core::{arch::global_asm, ptr::addr_of};

use crate::task::TaskPlugin;
use allocator::{Locked, bump::BumpAllocator};
use bevy::{
	DefaultPlugins,
	app::{App, PanicHandlerPlugin, Startup},
	prelude::{Commands, PluginGroup},
};
use linked_list_allocator::LockedHeap;
use talc::{ClaimOnOom, Span, Talc, Talck};

global_asm!(
	".section .text",
	".global _start",
	"_start:",
	"ldr x30, =stack_top",
	"mov sp, x30",
	"bl {main}",
	main = sym main,
);

extern crate alloc;

mod allocator;
mod critical_section;
mod mmio;
mod task;
mod uart;

const HEAP_SIZE: usize = 1000;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

fn main() {
	unsafe {
		ALLOCATOR.lock().init(&raw mut HEAP as *mut u8, HEAP_SIZE);
	}

	let result = App::new()
		.add_plugins((
			DefaultPlugins.build().disable::<PanicHandlerPlugin>(),
			TaskPlugin,
		))
		.add_systems(Startup, startup)
		.run();

	if result.is_error() {
		println!("Error: {:?}", result);
	}
}

fn startup(mut commands: Commands) {
	println!("Booting bevyOS...");

	commands.spawn(task::Process {
		id: 0,
		state: task::ProcessState::Starting,
	});
}
