use crate::mmio::mmio_write;
use core::fmt::{self, Write};

pub struct Uart;

impl Write for Uart {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		unsafe {
			mmio_write(0x900_0000, s.as_bytes());
		}

		Ok(())
	}
}

// Safe because it just contains a pointer to device memory, which can be
// accessed from any context.
unsafe impl Send for Uart {}

// TODO: ::io
// pub type Result<T> = core::result::Result<T, Error>;
// pub enum Error {
// WriteAllEof,
// Interrupted,
// }
// pub trait Write {
// fn write(&mut self, bytes: &[u8]) -> Result<usize>;
// fn write_all(&mut self, buf: &[u8]) -> Result<()>;
// fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<()>;
// }

#[macro_export]
macro_rules! println {
    () => {
    	use core::fmt::Write as _;
        let _ = write!($crate::uart::Uart, "\n");
    };
    ($($arg:tt)*) => {
    	use core::fmt::Write as _;
        let _ = write!($crate::uart::Uart, "{}\n", format_args!($($arg)*));
    };
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}
