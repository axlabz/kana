//! Helper functions used by the integration tests.

#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Iterator;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

/// Returns an iterator for the lines on a file.
pub fn read_lines<P: AsRef<Path>>(
	filename: P,
) -> io::Result<impl Iterator<Item = (String, usize)>> {
	let file = File::open(filename)?;
	Ok(BufReader::new(file)
		.lines()
		.enumerate()
		.map(|(pos, line)| {
			let line = line.unwrap();
			let line = line.trim();
			let line = match line.find("//") {
				Some(index) => line[..index].trim(),
				None => line,
			};
			(line.to_owned(), pos + 1)
		})
		.filter(|(x, _)| x.len() > 0))
}

/// A line of data from a character test data file.
pub struct TestChar {
	/// Source line number.
	pub line: usize,

	/// Data for the line.
	pub info: TestCharInfo,
}

#[derive(Debug)]
/// Information for a [`TestChar`].
pub enum TestCharInfo {
	/// Defines a single character.
	Single {
		char: char,
		code: String,
		description: String,
	},

	/// Defines a range of characters.
	Range {
		start: char,
		end: char,
		description: String,
	},

	/// Defines a list of characters.
	List(Vec<char>),

	/// Defines a list of flags for following characters.
	Flags(kana::Flags),
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
	let chars = lines.map(move |(line, n)| {
		//----[ Helpers ]-----------------------------------------------------//

		// Line and file information.
		let info = || format!("{}:{}: ", fileinfo, n);

		// Validates and returns the next split field.
		enum Next {
			Field(&'static str),
			End,
		}

		// Split the line into tabs and returns an iterator function.
		fn split_tabs<'a, F: Fn() -> String>(
			line: &'a str,
			info: F,
		) -> impl FnMut(Next) -> &'a str {
			let mut iter = line.trim().split('\t');
			let next = move |kind: Next| match kind {
				Next::Field(name) => match iter.next() {
					Some(x) => x,
					None => panic!("{}: expected {}", info(), name),
				},
				Next::End => {
					if let Some(_) = iter.next() {
						panic!("{}: unexpected extra field", info());
					} else {
						""
					}
				}
			};

			next
		}

		// Parses a UNICODE character code into a char.
		let parse_char = |input: &str| {
			if !RE_CHAR.is_match(input) {
				panic!("{}: invalid character code `{}`", info(), input);
			}

			let char: u32 = u32::from_str_radix(&input[2..], 16).unwrap();
			let char = match char::from_u32(char) {
				Some(x) => x,
				None => panic!("{}: invalid codepoint `{}`", info(), input),
			};

			char
		};

		//----[ Line parsing ]------------------------------------------------//

		let line = line.trim();
		let info = if line.len() == 0 {
			None
		} else if line.starts_with(":=") {
			let line = line[2..].trim();
			let flags = match line.parse::<kana::Flags>() {
				Ok(flags) => flags,
				Err(err) => panic!("{}: {}", info(), err),
			};
			Some(TestCharInfo::Flags(flags))
		} else if line.starts_with(">") {
			//----
			// Parse a literal list of characters
			//----
			let line = line[1..].trim();
			Some(TestCharInfo::List(
				line.chars().filter(|&x| x != ' ' && x != '\t').collect(),
			))
		} else if line.starts_with("~") {
			//----
			// Parse a tab-separated character range
			//----
			let mut read = split_tabs(&line[1..], info);
			let txt_start = read(Next::Field("range start"));
			let txt_end = read(Next::Field("range end"));
			let description = read(Next::Field("range description"));
			read(Next::End);

			let start = parse_char(txt_start);
			let end = parse_char(txt_end);
			if end <= start {
				panic!("{}: invalid range ({}..{})", info(), txt_start, txt_end);
			}

			let description = description.to_string();
			Some(TestCharInfo::Range {
				start,
				end,
				description,
			})
		} else {
			//----
			// Parse a tab-separated single character
			//----
			let mut read = split_tabs(line, info);
			let txt_code = read(Next::Field("character code"));
			let txt_char = read(Next::Field("character literal")).trim();
			let description = read(Next::Field("character description"));
			read(Next::End);

			// parse the character code, this is the one we really care about
			let char = parse_char(txt_code);

			// validate that the literal is at most a single char
			if txt_char.chars().count() > 1 {
				panic!(
					"{}: invalid character literal for {} (`{}`)",
					info(),
					txt_code,
					txt_char
				);
			}

			// if there is a literal, validate that it matches the code (the
			// literal is informative only)
			if let Some(char_literal) = txt_char.chars().next() {
				if char_literal != char {
					panic!(
						"{}: character literal does not match code ({} != `{}`)",
						info(),
						txt_code,
						txt_char
					);
				}
			}

			let code = txt_code.to_string();
			let description = description.to_string();
			Some(TestCharInfo::Single {
				char,
				code,
				description,
			})
		};

		match info {
			Some(info) => Some(TestChar { line: n, info }),
			None => None,
		}
	});
	let chars = chars.flatten();
	Ok(chars)
}
