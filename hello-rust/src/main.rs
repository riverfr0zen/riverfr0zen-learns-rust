use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
	let stdout = stdout();
	let out = b"Injecting some Rust into my veins";
	let width = 36;

	let mut writer = BufWriter::new(stdout.lock());
	say(out, width, &mut writer).unwrap();
}
