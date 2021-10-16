//! Simple benchmarking for the [kana::get_flags] function.

use std::{fs::File, io::Read, time::Instant};

extern crate kana;

fn run(filename: &'static str) {
	println!("\n>> Testing {}...", filename);

	// read the input file to memory:
	let filename = format!("./testdata/perf/{}", filename);
	let start = Instant::now();
	let mut file = File::open(filename).unwrap();
	let mut text = String::new();
	file.read_to_string(&mut text).unwrap();

	let elapsed = start.elapsed();
	println!("=> file read ({} bytes)\n   took {:?}", text.len(), elapsed,);

	//------------------------------------------------------------------------//

	// warm-up and establish a baseline
	let start = Instant::now();
	let mut cnt: u32 = 0;
	for _ in text.chars() {
		cnt += 1;
	}
	let elapsed = start.elapsed();
	println!("=> char count ({} chars)\n   took {:?}", cnt, elapsed);

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

fn main() {
	// read the input file to memory:
	run("kanjidic.dat");
	run("japanese.dat");
}
