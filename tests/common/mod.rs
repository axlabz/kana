//! Helper functions used by the integration tests.

#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Iterator;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

/// Returns an iterator for the lines on a file.
pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<impl Iterator<Item = String>> {
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

pub struct TestChar {
	pub char: char,
	pub code: String,
	pub description: String,
}

/// Returns an iterator for character test files.
pub fn read_chars<P: AsRef<Path> + std::fmt::Display>(
	filename: P,
) -> io::Result<impl Iterator<Item = TestChar>> {
	lazy_static! {
		static ref RE_CHAR: Regex = Regex::new(r"^U\+[\dA-F]{4,6}$").unwrap();
	}

	let fileinfo = format!("{}", filename);
	let lines = read_lines(filename)?;
	let chars = lines.map(move |x| {
		let mut iter = x.split('\t');
		let f1 = match iter.next() {
			Some(x) => x,
			None => panic!("{}: unexpected empty line", fileinfo),
		};
		let f2 = match iter.next() {
			Some(x) => x,
			None => panic!("{}: missing character information for {}", fileinfo, f1),
		};

		let (code, char, desc) = if let Some(desc) = iter.next() {
			if let Some(value) = iter.next() {
				panic!("{}: unexpected extra field `{}` at {}", fileinfo, value, f1);
			}
			(f1, f2, desc)
		} else {
			(f1, "", f2)
		};

		if !RE_CHAR.is_match(&code) {
			panic!("{}: invalid character code `{}`", fileinfo, code);
		}

		let code_chr: u32 = u32::from_str_radix(&code[2..], 16).unwrap();
		let code_chr = match char::from_u32(code_chr) {
			Some(x) => x,
			None => panic!("{}: invalid codepoint `{}`", fileinfo, char),
		};

		if char.len() > 0 {
			let chars = char.chars().collect::<Vec<char>>();
			if chars.len() > 1 {
				panic!("{}: invalid character `{}` at `{}`", fileinfo, char, code);
			} else if chars[0] != code_chr {
				panic!(
					"{}: character does not match code (`{}` != `{}`)",
					fileinfo, code, char
				);
			}
		}

		TestChar {
			char: code_chr,
			code: code.to_string(),
			description: desc.to_string(),
		}
	});
	Ok(chars)
}
