use kana_macros::charinfo;

#[test]
fn test_simple() {
	const A: u32 = 1;
	let info = charinfo!('a' => A);
	assert_eq!(info('a'), Some(A));
	assert_eq!(info('b'), None);
}
