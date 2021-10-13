//! Character testing functions.

use crate::{flag, get_flags};

/// Returns true if the given character is hiragana.
///
/// This includes the hiragana range, including rare characters, and the
/// long vowel mark.
pub fn is_hiragana(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is katakana.
///
/// This includes the whole katakana range, including rare and halfwidth
/// characters, and the long vowel mark.
pub fn is_katakana(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is hiragana or katakana.
pub fn is_kana(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is kanji.
pub fn is_kanji(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is a valid romaji character.
pub fn is_romaji(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is in the Japanese range.
pub fn is_japanese(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is a Japanese symbol. Does not include
/// punctuation.
pub fn is_symbol(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is a Japanese punctuation.
pub fn is_punctuation(_chr: char) -> bool {
	unimplemented!();
}

/// Returns true if the given character is [`SPACE`](crate::chars::Flags::SPACE).
pub fn is_space(chr: char) -> bool {
	get_flags(chr) & flag::SPACE
}
