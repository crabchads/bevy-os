use cc::Build;
use std::env;

fn main() {
	unsafe {
		env::set_var("CROSS_COMPILE", "aarch64-unknown-none-elf");
		env::set_var("CC", "clang");
	}

	Build::new()
		.file("src/entry.S")
		.file("src/exceptions.S")
		.file("src/idmap.S")
		.compile("empty")
}
