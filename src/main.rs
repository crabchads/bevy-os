#![no_main]
#![no_std]

mod critical_section;
mod exceptions;
mod logger;
mod pl011;
mod pl031;
mod task;

use crate::pl011::Uart;
use crate::pl031::Rtc;
use arm_gic::gicv3::GicV3;
use arm_gic::{IntId, Trigger, irq_enable};
use bevy::prelude::Commands;
use bevy::{
	DefaultPlugins,
	app::{App, PanicHandlerPlugin},
	prelude::{PluginGroup, Startup},
};
use buddy_system_allocator::LockedHeap;
use chrono::{TimeZone, Utc};
use core::panic::PanicInfo;
use log::{LevelFilter, error, info};
use smccc::Hvc;
use smccc::psci::system_off;
use task::TaskPlugin;

/// Base addresses of the GICv3.
const GICD_BASE_ADDRESS: *mut u64 = 0x800_0000 as _;
const GICR_BASE_ADDRESS: *mut u64 = 0x80A_0000 as _;

/// Base address of the primary PL011 UART.
const PL011_BASE_ADDRESS: *mut u32 = 0x900_0000 as _;
// ANCHOR_END: imports

/// Base address of the PL031 RTC.
const PL031_BASE_ADDRESS: *mut u32 = 0x901_0000 as _;
/// The IRQ used by the PL031 RTC.
const PL031_IRQ: IntId = IntId::spi(2);

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::new();

const HEAP_SIZE: usize = 0x100000; // 64K
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// SAFETY: There is no other global function of this name.
#[unsafe(no_mangle)]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) {
	// SAFETY: `HEAP` is only used here and `entry` is only called once.
	unsafe {
		// Give the allocator some memory to allocate.
		HEAP_ALLOCATOR
			.lock()
			.init(&raw mut HEAP as usize, HEAP_SIZE);
	}

	// SAFETY: `PL011_BASE_ADDRESS` is the base address of a PL011 device, and
	// nothing else accesses that address range.
	let uart = unsafe { Uart::new(PL011_BASE_ADDRESS) };
	logger::init(uart, LevelFilter::Trace).unwrap();

	info!("Booting bevyOS...");

	// SAFETY: `GICD_BASE_ADDRESS` and `GICR_BASE_ADDRESS` are the base
	// addresses of a GICv3 distributor and redistributor respectively, and
	// nothing else accesses those address ranges.
	let mut gic =
		unsafe { GicV3::new(GICD_BASE_ADDRESS, GICR_BASE_ADDRESS, 1, 0x20000) };
	gic.setup(0);

	GicV3::set_priority_mask(0xff);
	gic.set_interrupt_priority(PL031_IRQ, None, 0x80);
	gic.set_trigger(PL031_IRQ, None, Trigger::Level);
	irq_enable();
	gic.enable_interrupt(PL031_IRQ, None, true);

	let result = App::new()
		.add_plugins((
			DefaultPlugins.build().disable::<PanicHandlerPlugin>(),
			TaskPlugin,
		))
		.add_systems(Startup, startup)
		.run();

	if result.is_error() {
		error!("Error: {:?}", result);
	}

	system_off::<Hvc>().unwrap();
}

fn startup(mut commands: Commands) {
	commands.spawn(task::Process {
		id: 0,
		state: task::ProcessState::Starting,
	});
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	error!("{info}");
	system_off::<Hvc>().unwrap();
	loop {}
}
