//! Build test for [`charinfo!`].

use kana_macros::charinfo;

const A: u32 = 1;
const B: u32 = 2;
const C: u32 = 2;
const D: u32 = 2;

const NONE: u32 = 0;

fn main() {
	//----[ single char, single flag ]----------------------------------------//

	// single branch
	let _ = charinfo!(u32, 'a' => A);
	let _ = charinfo!(u32, 'a' => A,);

	// multiple branches
	let _ = charinfo!(u32, 'a' => A, 'b' => B);
	let _ = charinfo!(u32, 'a' => A, 'b' => B,);

	let _ = charinfo!(u32, 'a' => A, 'b' => B, 'c' => C);
	let _ = charinfo!(u32, 'a' => A, 'b' => B, 'c' => C,);

	let _ = charinfo!(u32,
		'a' => A,
		'b' => B,
		'c' => C,
		'd' => D);
	let _ = charinfo!(u32,
		'a' => A,
		'b' => B,
		'c' => C,
		'd' => D,
	);

	//----[ multi char, single flag ]----------------------------------------//

	// single branch
	let _ = charinfo!(u32, 'a'|'A' => A);
	let _ = charinfo!(u32, 'a'|'A' => A,);

	// multiple branches
	let _ = charinfo!(u32, 'a'|'A' => A, 'b'|'B' => B);
	let _ = charinfo!(u32, 'a'|'A' => A, 'b'|'B' => B,);

	let _ = charinfo!(u32, 'a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C);
	let _ = charinfo!(u32, 'a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C,);

	let _ = charinfo!(u32, 'a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C);
	let _ = charinfo!(u32, 'a'|'A' => A, 'b'|'B' => B, 'c'|'C' => C,);

	//----[ multi char, multi flag ]------------------------------------------//

	// single branch
	let _ = charinfo!(u32, 'a'|'b' => A | B);
	let _ = charinfo!(u32, 'a'|'b' => A | B,);

	// multiple branches
	let _ = charinfo!(u32, 'a'|'b' => A | B, 'b'|'c' => B | C);
	let _ = charinfo!(u32, 'a'|'b' => A | B, 'b'|'c' => B | C,);

	let _ = charinfo!(u32, 'a'|'b' => A | B, 'b'|'c' => B | C, 'd' => D);
	let _ = charinfo!(u32, 'a'|'b' => A | B, 'b'|'c' => B | C, 'd' => D,);

	//----[ char ranges ]-----------------------------------------------------//

	// single branch
	let _ = charinfo!(u32, 'a'..'b' => A | B);
	let _ = charinfo!(u32, 'a'..'b' => A | B,);

	// multiple branches
	let _ = charinfo!(u32, 'a'..'b' => A | B, 'b'..'c' => B | C);
	let _ = charinfo!(u32, 'a'..'b' => A | B, 'b'..'c' => B | C,);

	let _ = charinfo!(u32, 'a'..'b' => A | B, 'b'..'c' => B | C, 'd' => D);
	let _ = charinfo!(u32, 'a'..'b' => A | B, 'b'..'c' => B | C, 'd' => D,);

	//----[ inclusive ranges ]------------------------------------------------//

	// single branch
	let _ = charinfo!(u32, 'a'..='b' => A | B);
	let _ = charinfo!(u32, 'a'..='b' => A | B,);

	// multiple branches
	let _ = charinfo!(u32, 'a'..='b' => A | B, 'b'..='c' => B | C);
	let _ = charinfo!(u32, 'a'..='b' => A | B, 'b'..='c' => B | C,);

	let _ = charinfo!(u32, 'a'..='b' => A | B, 'b'..='c' => B | C, 'd' => D);
	let _ = charinfo!(u32, 'a'..='b' => A | B, 'b'..='c' => B | C, 'd' => D,);

	//----[ string ranges ]---------------------------------------------------//

	// single branch
	let _ = charinfo!(u32, "abc" => A | B | C);
	let _ = charinfo!(u32, "abc" => A | B | C,);

	// multiple branches
	let _ = charinfo!(u32, "ab" => A | B, "bc" => B | C);
	let _ = charinfo!(u32, "ab" => A | B, "bc" => B | C,);

	//----[ catch-all result ]------------------------------------------------//

	let _ = charinfo!(u32, * => NONE);
}
