//! Character tables

use kana_macros::charinfo;

use crate::{Flag, Flags};

/// Returns a set of the flags mapped for the given character. The flags are a
/// bitwise combination of the constants in the [`Flags`] namespace.
///
/// If the given character is not mapped, returns zero (i.e. [`Flags::NONE`]).
///
/// See the [`Flags`] namespace and individual flags for more details on the
/// mappings and rationale behind the flags.
pub fn flags(chr: char) -> Flags {
	let get = charinfo!(

		// basic ASCII spaces, including line breaks
		" \t\r\n" => Flag::SPACE,

		// space separator Unicode category
		'\u{00A0}' | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' |
		'\u{2003}' | '\u{2004}' | '\u{2005}' | '\u{2006}' | '\u{2007}' |
		'\u{2008}' | '\u{2009}' | '\u{200A}' | '\u{202F}' | '\u{205F}' |
		'\u{3000}' => Flag::SPACE,

		// additional Unicode line/paragraph separators
		'\u{2028}' | '\u{2029}' => Flag::SPACE,

		* => Flag::NONE,
	);

	get(chr)
}
