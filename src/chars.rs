//! Character tables

use kana_macros::charinfo;

/// Contais the flags provided by the [`flags`](super::flags) function.
#[allow(non_snake_case)]
pub mod Flags {
	/// Constant for no flags (zero).
	pub const NONE: u32 = 0;

	pub const HIRAGANA: u32 = 1 << 1;
	pub const KATAKANA: u32 = 1 << 2;
	pub const KANJI: u32 = 1 << 3;

	/// Space separator characters, including TAB and line separators.
	///
	/// This consists of:
	/// - `\x20`, `\t`, `\r`, and `\n` ASCII characters.
	/// - `Zs` "Space Separator" Unicode category.
	/// - `U+2028` and `U+2029` (Line and Paragraph separators).
	pub const SPACE: u32 = 1 << 4;
}

/// Returns a set of flags for the given character. The flags are a bitwise
/// combination of the constants in [`Flags`].
///
/// If the given character is not mapped, returns zero (i.e. [`Flags::NONE`]).
pub fn flags(chr: char) -> u32 {
	let get = charinfo!(

		// basic ASCII spaces, including line breaks
		" \t\r\n" => Flags::SPACE,

		// space separator Unicode category
		'\u{00A0}' | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' |
		'\u{2003}' | '\u{2004}' | '\u{2005}' | '\u{2006}' | '\u{2007}' |
		'\u{2008}' | '\u{2009}' | '\u{200A}' | '\u{202F}' | '\u{205F}' |
		'\u{3000}' => Flags::SPACE,

		// additional Unicode line/paragraph separators
		'\u{2028}' | '\u{2029}' => Flags::SPACE,

		* => Flags::NONE,
	);

	get(chr)
}
