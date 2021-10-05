//! Build test for [`charinfo!`].

use kana_macros::charinfo;

const A: u32 = 1;
const B: u32 = 2;
const C: u32 = 2;
const D: u32 = 2;

fn main() {
	//----[ single char, single flag ]----------------------------------------//

	// single branch
	let _ = charinfo!('a' => A);
	let _ = charinfo!('a' => A,);

	// multiple branches
	let _ = charinfo!('a' => A, 'b' => B);
	let _ = charinfo!('a' => A, 'b' => B,);

	let _ = charinfo!('a' => A, 'b' => B, 'c' => C);
	let _ = charinfo!('a' => A, 'b' => B, 'c' => C,);

	let _ = charinfo!(
		'a' => A,
		'b' => B,
		'c' => C,
		'd' => D);
	let _ = charinfo!(
		'a' => A,
		'b' => B,
		'c' => C,
		'd' => D,
	);

	//----[ multi char, single flag ]----------------------------------------//

	// single branch
	let _ = charinfo!('a'|'A' => A);
	let _ = charinfo!('a'|'A' => A,);

	// multiple branches
	let _ = charinfo!('a'|'A' => A, 'b'|'B' => B);
	let _ = charinfo!('a'|'A' => A, 'b'|'B' => B,);

	let _ = charinfo!('a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C);
	let _ = charinfo!('a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C,);

	let _ = charinfo!('a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C);
	let _ = charinfo!('a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C,);

	//----[ multi char, multi flag ]------------------------------------------//

	// single branch
	let _ = charinfo!('a'|'b' => A | B);
	let _ = charinfo!('a'|'b' => A | B,);

	// multiple branches
	let _ = charinfo!('a'|'b' => A | B, 'b'|'c' => B | C);
	let _ = charinfo!('a'|'b' => A | B, 'b'|'c' => B | C,);

	let _ = charinfo!('a'|'b' => A | B, 'b'|'c' => B | C, 'd' => D);
	let _ = charinfo!('a'|'b' => A | B, 'b'|'c' => B | C, 'd' => D,);

	//----[ char ranges ]-----------------------------------------------------//

	// single branch
	let _ = charinfo!('a'..'b' => A | B);
	let _ = charinfo!('a'..'b' => A | B,);

	// multiple branches
	let _ = charinfo!('a'..'b' => A | B, 'b'..'c' => B | C);
	let _ = charinfo!('a'..'b' => A | B, 'b'..'c' => B | C,);

	let _ = charinfo!('a'..'b' => A | B, 'b'..'c' => B | C, 'd' => D);
	let _ = charinfo!('a'..'b' => A | B, 'b'..'c' => B | C, 'd' => D,);

	//----[ inclusive ranges ]------------------------------------------------//

	// single branch
	let _ = charinfo!('a'..='b' => A | B);
	let _ = charinfo!('a'..='b' => A | B,);

	// multiple branches
	let _ = charinfo!('a'..='b' => A | B, 'b'..='c' => B | C);
	let _ = charinfo!('a'..='b' => A | B, 'b'..='c' => B | C,);

	let _ = charinfo!('a'..='b' => A | B, 'b'..='c' => B | C, 'd' => D);
	let _ = charinfo!('a'..='b' => A | B, 'b'..='c' => B | C, 'd' => D,);
}