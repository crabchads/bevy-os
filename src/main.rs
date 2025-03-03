#![feature(naked_functions)]
#![no_main]
#![no_std]

use core::arch::naked_asm;

use crate::task::TaskPlugin;
use bevy::{
	DefaultPlugins,
	app::{App, PanicHandlerPlugin, Startup},
	prelude::{Commands, PluginGroup},
};

#[naked]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
	unsafe {
		naked_asm!(
			"ldr x30, =stack_top",
			"mov sp, x30",
			"bl {main}",
			main = sym main,
		)
	}
}

extern crate alloc;

use linked_list_allocator::LockedHeap;

mod allocator;
mod critical_section;
mod task;
mod uart;

// Define the heap size in bytes
const HEAP_SIZE: usize = 1024 * 1024; 

// Static memory for the heap
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

fn main() {
	unsafe {
		ALLOCATOR.lock().init(
			&raw mut HEAP as *mut u8,
			HEAP_SIZE,
		);
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
	commands.spawn(task::Process {
		id: 0,
		state: task::ProcessState::Starting,
	});
}
