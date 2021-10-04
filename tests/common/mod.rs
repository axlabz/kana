//! Helper functions used by the integration tests.

#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Returns an iterator for the lines on a file.
pub fn read_lines<P: AsRef<Path>>(
	filename: P,
) -> io::Result<impl std::iter::Iterator<Item = String>> {
	let file = File::open(filename)?;
	Ok(BufReader::new(file)
		.lines()
		.map(|x| {
			let line = x.unwrap();
			let line = line.trim();
			let line = match line.find('#') {
				Some(index) => line[..index].trim(),
				None => line,
			};
			line.to_owned()
		})
		.filter(|x| x.len() > 0))
}
