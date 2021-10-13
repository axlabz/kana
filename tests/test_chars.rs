mod common;

use common::{TestChar, TestCharInfo};
use kana::{flag, get_flags, Flags};

#[test]
fn test_flags() -> Result<(), String> {
	let chars = common::read_chars("./testdata/chars/flags.in").unwrap();

	let mut expected = (flag::NONE, 0);

	fn check_char<F: Fn() -> String>(
		char: char,
		line: usize,
		(expected, expected_line): (Flags, usize),
		info: F,
	) -> bool {
		let flags = get_flags(char);
		if flags != expected {
			eprintln!(
				"fail: character U+{:04X} at L{:03} differs from expectation (L{:03})\n      expected {}, was {}{}",
				char as u32,
				line,
				expected_line,
				expected,
				flags,
				{
					let info = info();
					if info.len() > 0 {
						format!("\n      {}", info)
					} else {
						info
					}
				},
			);
			false
		} else {
			true
		}
	}

	const MAX_FAILS: usize = 32;

	let mut fails = 0;
	for TestChar { line, info } in chars.into_iter() {
		if fails >= MAX_FAILS {
			eprintln!("fail: too many fails, bailing out");
			break;
		}

		match info {
			TestCharInfo::Flags(flags) => {
				expected = (flags, line);
			}
			TestCharInfo::List(chars) => {
				for char in chars.into_iter() {
					if !check_char(char, line, expected, || String::new()) {
						fails += 1;
						if fails >= MAX_FAILS {
							break;
						}
					}
				}
			}
			TestCharInfo::Single {
				char,
				code,
				description,
			} => {
				if !check_char(char, line, expected, || {
					format!("{} - {}", code, description)
				}) {
					fails += 1;
				}
			}
			TestCharInfo::Range {
				start,
				end,
				description,
			} => {
				for char in start..=end {
					if !check_char(char, line, expected, || format!("{}", description)) {
						fails += 1;
						if fails >= MAX_FAILS {
							break;
						}
					};
				}
			}
		}
	}

	if fails > 0 {
		Err(format!("failed {} checks", fails))
	} else {
		Ok(())
	}
}
