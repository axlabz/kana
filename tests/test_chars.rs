mod common;

use std::collections::HashSet;

use common::{TestChar, TestCharInfo};
use kana::{flag, get_flags, Flags};

/// Test the `flags.in` table containing the expected flags for all mapped
/// characters.
///
/// This does an exact test of all flags per character, and also tests
/// characters outside the range to make sure they are unmapped (and that
/// the `flags.in` file is not missing any character).
#[test]
fn all_flags() -> Result<(), String> {
	// parse the test input
	let chars = common::read_chars("./testdata/chars/flags.in").unwrap();

	// this contains the current expectation as we process the input file
	let mut expected = (flag::NONE, 0);

	// after this many fails we bail out
	const MAX_FAILS: usize = 32;

	// this contains all characters that have a mapping from the input
	let mut charset: HashSet<char> = HashSet::new();

	// current number of failures
	let mut fails = 0;

	// process all lines in the input
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
					if !check_char(&mut charset, char, line, expected, || String::new()) {
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
				if !check_char(&mut charset, char, line, expected, || {
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
					if !check_char(&mut charset, char, line, expected, || {
						format!("{}", description)
					}) {
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
		return Err(format!("failed {} checks", fails));
	}

	// check that unmapped characters return NONE
	for char in '\0'..='\u{2FFFF}' {
		if !charset.contains(&char) {
			assert_eq!(
				get_flags(char),
				flag::NONE,
				"U+{:04X} should not be mapped",
				char as u32
			);
		}
	}

	return Ok(());

	/// Helper to test a single character expectation.
	fn check_char<F: Fn() -> String>(
		charset: &mut HashSet<char>,
		char: char,
		line: usize,
		(expected, expected_line): (Flags, usize),
		info: F,
	) -> bool {
		let added = charset.insert(char);
		let flags = get_flags(char);
		let (valid, mode) = if added {
			(flags == expected, "")
		} else {
			// if this is a duplicated expectation, just check that it contains
			// the expected flags (useful to test separate flags for specific
			// characters)
			(flags & expected, " (contains)")
		};
		if !valid {
			eprintln!(
				"fail: character U+{:04X} at L{:03} differs from expectation (L{:03})\n      expected {}, was {}{}{}",
				char as u32,
				line,
				expected_line,
				expected,
				flags,
				mode,
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
}

//----------------------------------------------------------------------------//
// Flag-specific tests
//----------------------------------------------------------------------------//

// Tests below check that the given characters contain a specific flag. This
// provides a second redundant test in relation to `flags.in` above.

#[test]
fn fullwidth() {
	check_flags(Check::Contains, "fullwidth.in", flag::FULLWIDTH);
}

#[test]
fn halfwidth() {
	check_flags(Check::Contains, "halfwidth.in", flag::HALFWIDTH);
}

#[test]
fn hiragana() {
	check_flags(Check::Contains, "hiragana.in", flag::HIRAGANA);
}

#[test]
fn kana() {
	check_flags(Check::Contains, "kana.in", flag::KANA);
}

#[test]
fn katakana() {
	check_flags(Check::Contains, "katakana.in", flag::KATAKANA);
}

#[test]
fn katakana_halfwidth() {
	check_flags(
		Check::Contains,
		"katakana-halfwidth.in",
		flag::HALFWIDTH | flag::KATAKANA,
	);
}

#[test]
fn linebreak() {
	check_flags(Check::Contains, "linebreak.in", flag::LINEBREAK);
}

#[test]
fn number() {
	check_flags(Check::Contains, "number.in", flag::NUMBER);
}

#[test]
fn punctuation_japanese() {
	check_flags(
		Check::Contains,
		"punctuation-japanese.in",
		flag::PUNCTUATION | flag::JAPANESE,
	);
}

#[test]
fn punctuation_romaji() {
	check_flags(
		Check::Contains,
		"punctuation-romaji.in",
		flag::PUNCTUATION | flag::ROMAJI,
	);
}

#[test]
fn rare() {
	check_flags(Check::Contains, "rare.in", flag::RARE);
}

#[test]
fn romaji() {
	check_flags(Check::Contains, "romaji.in", flag::ROMAJI);
}

#[test]
fn roman() {
	check_flags(Check::Contains, "roman.in", flag::ROMAN);
}

#[test]
fn small() {
	check_flags(Check::Contains, "small.in", flag::SMALL);
}

#[test]
fn space() {
	check_flags(Check::Contains, "space.in", flag::SPACE);
}

#[test]
fn symbol_japanese() {
	check_flags(
		Check::Contains,
		"symbol-japanese.in",
		flag::SYMBOL | flag::JAPANESE,
	);
}

#[test]
fn symbol_romaji() {
	check_flags(
		Check::Contains,
		"symbol-romaji.in",
		flag::SYMBOL | flag::ROMAJI,
	);
}

#[test]
fn word() {
	check_flags(Check::Contains, "word.in", flag::WORD);
	check_flags(Check::Contains, "hiragana.in", flag::WORD);
	check_flags(Check::Contains, "katakana.in", flag::WORD);
	check_flags(Check::Contains, "kana.in", flag::WORD);
	check_flags(Check::Contains, "katakana-halfwidth.in", flag::WORD);
	check_flags(Check::Contains, "roman.in", flag::WORD);
}

#[test]
fn word_are_either_romaji_or_japanese() {
	assert_all(|_, flags| {
		if flags & flag::WORD {
			flags & flag::JAPANESE || flags & flag::ROMAJI
		} else {
			true
		}
	})
}

//----------------------------------------------------------------------------//
// Helper code
//----------------------------------------------------------------------------//

#[allow(dead_code)]
enum Check {
	Equal,
	Contains,
}

/// Test that the given expected flags are equal or contained in all characters
/// from the testdata character file.
fn check_flags(kind: Check, file: &'static str, expected: Flags) {
	let filename = format!("./testdata/chars/{}", file);
	let chars = common::read_chars(&filename).unwrap();

	let check = |char: char, line: usize, description: &str| {
		let actual = get_flags(char);
		let info = || {
			format!(
				"({}, line {}, U+{:04X}{}{})",
				file,
				line,
				char as u32,
				if description.len() > 0 { " " } else { "" },
				description
			)
		};
		match kind {
			Check::Equal => assert_eq!(
				actual,
				expected,
				"expected {}, but it was {} {}",
				expected,
				actual,
				info(),
			),
			Check::Contains => assert!(
				actual & expected,
				"expected to contain {}, but it was {} {}",
				expected,
				actual,
				info(),
			),
		}
	};

	let mut count = 0;
	for TestChar { line, info } in chars.into_iter() {
		count += 1;
		match info {
			TestCharInfo::Single {
				char, description, ..
			} => check(char, line, &description),
			TestCharInfo::Range {
				start,
				end,
				description,
			} => {
				for char in start..=end {
					check(char, line, &description)
				}
			}
			TestCharInfo::List(chars) => {
				for char in chars.into_iter() {
					check(char, line, "")
				}
			}
			_ => {
				panic!("{}:{}: syntax not supported: {:?}", file, line, info);
			}
		}
	}

	assert!(count > 0, "{}: tested no characters", filename);
}

/// Check that the given condition is valid for all mapped character.
fn assert_all<F: Fn(char, Flags) -> bool>(condition: F) {
	// parse the test input
	let chars = common::read_chars("./testdata/chars/flags.in").unwrap();

	// process all lines in the input
	for TestChar { line, info } in chars.into_iter() {
		match info {
			TestCharInfo::Flags(_) => {
				// for this test we ignore expected flags
			}
			TestCharInfo::List(chars) => {
				for char in chars.into_iter() {
					check_char(&condition, char, line, || String::new());
				}
			}
			TestCharInfo::Single {
				char,
				code,
				description,
			} => {
				check_char(&condition, char, line, || {
					format!("{} - {}", code, description)
				});
			}
			TestCharInfo::Range {
				start,
				end,
				description,
			} => {
				for char in start..=end {
					check_char(&condition, char, line, || format!("{}", description));
				}
			}
		}
	}

	/// Helper to test a single character expectation.
	fn check_char<F: Fn(char, Flags) -> bool, Info: Fn() -> String>(
		condition: &F,
		char: char,
		line: usize,
		info: Info,
	) {
		let flags = get_flags(char);
		if !condition(char, flags) {
			panic!(
				"condition failed for U+{:04X} at flags.in:{:03} ({}){}",
				char as u32,
				line,
				flags,
				{
					let info = info();
					if info.len() > 0 {
						format!("\n      {}", info)
					} else {
						info
					}
				}
			);
		}
	}
}
