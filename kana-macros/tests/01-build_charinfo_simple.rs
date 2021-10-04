use kana_macros::charinfo;

const A: u32 = 1;
const B: u32 = 2;

fn main() {
	let f = charinfo!(
		'a' => A,
		'b' => B,
	);
	f('a');
	f('b');
}
