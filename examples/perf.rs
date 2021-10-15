//! Simple benchmarking for the [kana::get_flags] function.

use std::{fs::File, io::Read, time::Instant};

extern crate kana;

fn main() {
	// read the input file to memory:

	let start = Instant::now();
	let mut file = File::open("./testdata/perf/kanjidic.dat").unwrap();
	let mut text = String::new();
	file.read_to_string(&mut text).unwrap();

	let elapsed = start.elapsed();
	println!("=> file read ({} bytes)\n   took {:?}", text.len(), elapsed,);

	//------------------------------------------------------------------------//

	// warm-up and establish a baseline
	let start = Instant::now();
	let mut sum: u32 = 0;
	let mut cnt: u32 = 0;
	for chr in text.chars() {
		sum += chr as u32;
		cnt += 1;
	}
	let elapsed = start.elapsed();
	println!(
		"=> plain sum ({} chars = {})\n   took {:?}",
		cnt, sum, elapsed
	);

	//------------------------------------------------------------------------//

	// compare the plain bitwise baseline time
	let start = Instant::now();
	let mut flags: u32 = 0;
	for chr in text.chars() {
		flags = flags | chr as u32;
	}
	let elapsed = start.elapsed();
	println!("=> plain bitwise ({})\n   took {:?}", flags, elapsed);

	//------------------------------------------------------------------------//

	// test the character flags performance
	let start = Instant::now();
	let mut flags: kana::Flags = kana::Flags::default();
	for chr in text.chars() {
		flags = flags | kana::get_flags(chr);
	}
	let elapsed = start.elapsed();
	println!("=> get_flags ({})\n   took {:?}", flags, elapsed);
}
