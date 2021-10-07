//! Self-check for the integration tests.

mod common;

/// Test that we can call the library functions.
#[test]
fn test_works() {
	assert_eq!(kana::answer(), 42);
}

/// Test that we can read the `testdata` files and that empty lines and
/// comments are properly ignored.
#[test]
fn test_can_read_testdata() {
	let lines: Vec<String> = common::read_lines("./testdata/self_check.in")
		.unwrap()
		.collect();
	assert_eq!(lines, vec!["line 1", "line 2"]);
}

/// Test that we can read and parse the `testdata` character tables.
#[test]
fn test_can_read_chardata() {
	let chars: Vec<_> = common::read_chars("./testdata/chars/hiragana.in")
		.unwrap()
		.collect();
	chars
		.iter()
		.find(|it| it.char == 'あ' && it.description.to_lowercase() == "hiragana letter a")
		.expect("should have 'あ'");

	let chars: Vec<_> = common::read_chars("./testdata/chars/spaces.in")
		.unwrap()
		.collect();
	chars
		.iter()
		.find(|it| it.char == ' ' && it.description.to_lowercase() == "space (sp)")
		.expect("should have the space character");
}
