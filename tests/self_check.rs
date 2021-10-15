//! Self-check for the integration tests.

use common::{TestChar, TestCharInfo};

mod common;

/// Test that we can read the `testdata` files and that empty lines and
/// comments are properly ignored.
#[test]
fn test_can_read_testdata() {
	let lines: Vec<String> = common::read_lines("./testdata/self_check.in")
		.unwrap()
		.map(|(x, _)| x)
		.collect();
	assert_eq!(lines, vec!["line 1", "line 2"]);
}

/// Test that we can read and parse the `testdata` character tables.
#[test]
fn test_can_read_chardata() {
	let chars: Vec<_> = common::read_chars("./testdata/chars/flags.in")
		.unwrap()
		.collect();
	chars
		.iter()
		.find(|TestChar { info, .. }| {
			if let TestCharInfo::Single {
				char, description, ..
			} = info
			{
				*char == 'あ' && description.to_lowercase() == "hiragana letter a"
			} else {
				false
			}
		})
		.expect("should have 'あ'");
}
