//! Character testing functions.

use crate::{flag, get_flags};

/// Returns true if the given character is hiragana.
///
/// This includes the hiragana range, including rare characters, and the
/// long vowel mark.
pub fn is_hiragana(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::HIRAGANA || flags & (flag::KANA)
}

/// Returns true if the given character is katakana.
///
/// This includes the whole katakana range, including rare and halfwidth
/// characters, and the long vowel mark.
pub fn is_katakana(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::KATAKANA || flags & (flag::KANA)
}

/// Returns true if the given character is hiragana or katakana.
pub fn is_kana(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::HIRAGANA || flags & flag::KATAKANA || flags & (flag::KANA)
}

/// Returns true if the given character is kanji.
pub fn is_kanji(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::KANJI
}

/// Returns true if the given character is a valid romaji character.
pub fn is_romaji(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::ROMAJI
}

/// Returns true if the given character is in the Japanese range.
pub fn is_japanese(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::JAPANESE
}

/// Returns true if the given character is a word character.
pub fn is_word(chr: char) -> bool {
	let flags = get_flags(chr);
	flags & flag::WORD
}

/// Returns true if the given character is [`SPACE`](crate::chars::Flags::SPACE).
pub fn is_space(chr: char) -> bool {
	get_flags(chr) & flag::SPACE
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hiragana() {
		assert!(is_hiragana('あ'));
		assert!(is_hiragana('ー')); // long vowel mark
		assert!(is_hiragana('－')); // fullwidth hyphen
		assert!(is_hiragana('・'));
		assert!(!is_hiragana('x'));
		assert!(!is_hiragana('ア'));
	}

	#[test]
	fn katakana() {
		assert!(is_katakana('ア'));
		assert!(is_katakana('ー')); // long vowel mark
		assert!(is_katakana('－')); // fullwidth hyphen
		assert!(is_katakana('・'));
		assert!(!is_katakana('x'));
		assert!(!is_katakana('あ'));
	}

	#[test]
	fn kana() {
		assert!(is_kana('あ'));
		assert!(is_kana('ア'));
		assert!(is_kana('ー')); // long vowel mark
		assert!(is_kana('－')); // fullwidth hyphen
		assert!(is_kana('・'));
		assert!(!is_kana('x'));
	}

	#[test]
	fn kanji() {
		assert!(is_kanji('人'));
		assert!(is_kanji('日'));
		assert!(!is_kanji('x'));
	}

	#[test]
	fn space() {
		assert!(is_space(' '));
		assert!(is_space('\t'));
		assert!(is_space('\n'));
		assert!(is_space('\r'));
		assert!(is_space('　'));
		assert!(!is_space('x'));
	}

	#[test]
	fn romaji() {
		assert!(is_romaji('a'));
		assert!(is_romaji('A'));
		assert!(is_romaji('0'));
		assert!(is_romaji('-'));
		assert!(is_romaji('/'));
		assert!(is_romaji('·')); // U+00B7 Middle Dot
		assert!(!is_romaji(' '));
	}

	#[test]
	fn japanese() {
		assert!(is_japanese('あ'));
		assert!(is_japanese('ア'));
		assert!(is_japanese('日'));
		assert!(is_japanese('～'));
		assert!(is_japanese('０'));
		assert!(is_japanese('ー'));
		assert!(is_japanese('－'));
		assert!(is_japanese('・'));
		assert!(is_japanese('〇'));
		assert!(is_japanese('〶'));
		assert!(!is_japanese('x'));
		assert!(!is_japanese('　')); // ideographic space
	}
}
