mod common;

#[test]
fn is_space() {
	assert!(!kana::is_space('x'), "`x` is not a space");

	let chars = common::read_chars("./testdata/chars/spaces.in").unwrap();
	for it in chars {
		assert!(kana::is_space(it.char), "expected {} to be space", it.code);
	}
}
