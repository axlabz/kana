mod common;

use common::{TestChar, TestCharInfo};
use kana::is_space;

#[test]
fn test_is_space() {
	assert!(!is_space('x'), "`x` is not a space");

	let chars = common::read_chars("./testdata/chars/spaces.in").unwrap();
	for TestChar { info, .. } in chars.into_iter() {
		match info {
			TestCharInfo::Single { char, code, .. } => {
				assert!(kana::is_space(char), "expected {} to be space", code);
			}
			_ => panic!("not supported: {:?}", info),
		}
	}
}
