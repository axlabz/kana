use bitflags::bitflags;
use kana_macros::charinfo;

bitflags! {
	struct Flag: u32 {
		const A = 1 << 0;
		const B = 1 << 1;
		const C = 1 << 2;
		const X = 1 << 3;
	}
}

#[test]
fn with_bitflags() {
	let info = charinfo!(Flag,
		'a' => Flag::A,
		'b' => Flag::B,
		'c' => Flag::C,
		'a'..'c' => Flag::X,
		'd' => Flag::A | Flag::B | Flag::C,
	);

	assert_eq!(info('a'), Some(Flag::A | Flag::X));
	assert_eq!(info('b'), Some(Flag::B | Flag::X));
	assert_eq!(info('c'), Some(Flag::C));
	assert_eq!(info('d'), Some(Flag::A | Flag::B | Flag::C));
}
