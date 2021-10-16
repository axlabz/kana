use kana_macros::charinfo;

/// Test the simplest case.
#[test]
fn simple() {
	const A: u32 = 1;
	let info = charinfo!(u32, 'a' => A);
	assert_eq!(info('a'), Some(A));
	assert_eq!(info('b'), None);
}

/// Test using inclusive ranges.
#[test]
fn range_inclusive() {
	const UPPER: u32 = 1;
	const LOWER: u32 = 2;
	const DIGIT: u32 = 3;

	let info = charinfo!(u32,
		'a'..='z' => LOWER,
		'A'..='Z' => UPPER,
		'0'..='9' => DIGIT,
	);

	assert_eq!(info('a'), Some(LOWER));
	assert_eq!(info('b'), Some(LOWER));
	assert_eq!(info('y'), Some(LOWER));
	assert_eq!(info('z'), Some(LOWER));

	assert_eq!(info('A'), Some(UPPER));
	assert_eq!(info('B'), Some(UPPER));
	assert_eq!(info('Y'), Some(UPPER));
	assert_eq!(info('Z'), Some(UPPER));

	assert_eq!(info('0'), Some(DIGIT));
	assert_eq!(info('1'), Some(DIGIT));
	assert_eq!(info('8'), Some(DIGIT));
	assert_eq!(info('9'), Some(DIGIT));
}

/// Test exclusive ranges.
#[test]
fn range_exclusive() {
	const UPPER: u32 = 1;
	const LOWER: u32 = 2;
	const DIGIT: u32 = 3;

	let info = charinfo!(u32,
		'a'..'z' => LOWER,
		'A'..'Z' => UPPER,
		'0'..'9' => DIGIT,
	);

	assert_eq!(info('a'), Some(LOWER));
	assert_eq!(info('b'), Some(LOWER));
	assert_eq!(info('y'), Some(LOWER));
	assert_eq!(info('z'), None);

	assert_eq!(info('A'), Some(UPPER));
	assert_eq!(info('B'), Some(UPPER));
	assert_eq!(info('Y'), Some(UPPER));
	assert_eq!(info('Z'), None);

	assert_eq!(info('0'), Some(DIGIT));
	assert_eq!(info('1'), Some(DIGIT));
	assert_eq!(info('8'), Some(DIGIT));
	assert_eq!(info('9'), None);
}

/// Test multiple flags per character.
#[test]
fn multiple_flags() {
	const A: u32 = 1;
	const B: u32 = 2;
	const C: u32 = 4;
	const D: u32 = 8;
	const X: u32 = 16;

	let info = charinfo!(u32,
		'0' => A,
		'1' => B | C,
		'2' => C,
		'2' => D,
		'3' => A | B | C | D,
		'1'..'3' => X,
	);

	assert_eq!(info('0'), Some(A));
	assert_eq!(info('1'), Some(X | B | C));
	assert_eq!(info('2'), Some(X | C | D));
	assert_eq!(info('3'), Some(A | B | C | D));
}

/// Test a complex example.
#[test]
fn complex() {
	const ALPHA: u32 = 1 << 0;
	const DIGIT: u32 = 1 << 1;
	const UPPER: u32 = 1 << 2;
	const LOWER: u32 = 1 << 3;
	const SPACE: u32 = 1 << 4;
	const IDENT: u32 = 1 << 5;
	const EXTRA: u32 = 1 << 6;

	let info = charinfo!(u32,
		'0'..='9' => DIGIT,
		'A'..='Z' => ALPHA | UPPER,
		'a'..='z' => ALPHA | LOWER,
		' ' | '\t' => SPACE,
		'\r' | '\n' => SPACE,
		'_' => IDENT,
		'a'..='z' | 'A'..='Z' | '0'..='9' => IDENT,
		'\0'..'\t' => EXTRA,
	);

	assert_eq!(info('~'), None);
	assert_eq!(info('0'), Some(DIGIT | IDENT));
	assert_eq!(info('9'), Some(DIGIT | IDENT));
	assert_eq!(info('A'), Some(ALPHA | UPPER | IDENT));
	assert_eq!(info('Z'), Some(ALPHA | UPPER | IDENT));
	assert_eq!(info('a'), Some(ALPHA | LOWER | IDENT));
	assert_eq!(info('z'), Some(ALPHA | LOWER | IDENT));
	assert_eq!(info('_'), Some(IDENT));
	assert_eq!(info(' '), Some(SPACE));
	assert_eq!(info('\t'), Some(SPACE));
	assert_eq!(info('\r'), Some(SPACE));
	assert_eq!(info('\n'), Some(SPACE));
	assert_eq!(info('\0'), Some(EXTRA));
	assert_eq!(info('\x08'), Some(EXTRA));
}

/// Test the complex example but using the string syntax.
#[test]
fn complex_strings() {
	const ALPHA: u32 = 1 << 0;
	const DIGIT: u32 = 1 << 1;
	const UPPER: u32 = 1 << 2;
	const LOWER: u32 = 1 << 3;
	const SPACE: u32 = 1 << 4;
	const IDENT: u32 = 1 << 5;
	const EXTRA: u32 = 1 << 6;

	let info = charinfo!(u32,
		"0123456789" => DIGIT,
		"ABCDEFGHIJKLMNOPQRSTUVWXYZ" => ALPHA | UPPER,
		"abcdefghijklmnopqrstuvwxyz" => ALPHA | LOWER,
		" \t" => SPACE,
		"\r\n" => SPACE,
		"_" => IDENT,
		"abcdefghijklmnopqrstuvwxyz" | "ABCDEFGHIJKLMNOPQRSTUVWXYZ" | "0123456789" => IDENT,
		"\0\x01\x02\x03\x04\x05\x06\x07\x08" => EXTRA,
	);

	assert_eq!(info('~'), None);
	assert_eq!(info('0'), Some(DIGIT | IDENT));
	assert_eq!(info('9'), Some(DIGIT | IDENT));
	assert_eq!(info('A'), Some(ALPHA | UPPER | IDENT));
	assert_eq!(info('Z'), Some(ALPHA | UPPER | IDENT));
	assert_eq!(info('a'), Some(ALPHA | LOWER | IDENT));
	assert_eq!(info('z'), Some(ALPHA | LOWER | IDENT));
	assert_eq!(info('_'), Some(IDENT));
	assert_eq!(info(' '), Some(SPACE));
	assert_eq!(info('\t'), Some(SPACE));
	assert_eq!(info('\r'), Some(SPACE));
	assert_eq!(info('\n'), Some(SPACE));
	assert_eq!(info('\0'), Some(EXTRA));
	assert_eq!(info('\x06'), Some(EXTRA));
}

/// Test the catch-all rule.
#[test]
fn catch_all() {
	const A: u32 = 1;
	const X: u32 = 2;
	let info = charinfo!(u32,
		'a' => A,
		* => X,
	);
	assert_eq!(info('a'), A);
	assert_eq!(info('b'), X);
	assert_eq!(info('c'), X);
}

/// Test with chars outside the lookup range.
#[test]
fn large_chars() {
	const A: u32 = 1;
	const B: u32 = 2;

	let info = charinfo!(u32, '\u{1FFF0}'..'\u{1FFF4}' => A);

	assert_eq!(info('a'), None);
	assert_eq!(info('\u{1FFF0}'), Some(A));
	assert_eq!(info('\u{1FFF1}'), Some(A));
	assert_eq!(info('\u{1FFF2}'), Some(A));
	assert_eq!(info('\u{1FFF3}'), Some(A));
	assert_eq!(info('\u{1FFF4}'), None);

	let info = charinfo!(u32, '\u{1FFF0}'..'\u{1FFF4}' => A, 'b' => B);

	assert_eq!(info('a'), None);
	assert_eq!(info('b'), Some(B));
	assert_eq!(info('\u{1FFF0}'), Some(A));
	assert_eq!(info('\u{1FFF3}'), Some(A));
	assert_eq!(info('\u{1FFF4}'), None);
}
