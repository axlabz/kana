//! Kana manipulation library.
//!
//! Provides conversion between hiragana, katakana, and romaji, as well as
//! helper functions related to Japanese characters.

#![feature(type_alias_impl_trait)]
#![feature(test)]

extern crate kana_macros;

extern crate test;

mod flags;
pub use flags::{flag, Flags};

mod chars;
pub use chars::get_flags;

mod is;
pub use is::*;

mod convert;
pub use convert::*;

pub mod transform;

/// For word characters, returns either [`flag::JAPANESE`] or [`flag::ROMAJI`]
/// depending on the type of word.
pub fn word_type(chr: char) -> Option<Flags> {
	let flags = get_flags(chr);
	if flags & flag::WORD {
		if flags & flag::ROMAJI {
			Some(flag::ROMAJI)
		} else {
			Some(flag::JAPANESE)
		}
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use crate::get_flags;

	use super::*;
	use test::Bencher;

	#[test]
	fn word_type_is_valid() {
		assert_eq!(word_type('~'), None, "~");
		assert_eq!(word_type('_'), None, "_");
		assert_eq!(word_type('〶'), None, "〶");
		assert_eq!(word_type('a'), Some(flag::ROMAJI), "a");
		assert_eq!(word_type('1'), Some(flag::ROMAJI), "1");
		assert_eq!(word_type('-'), Some(flag::ROMAJI), "-");
		assert_eq!(word_type('/'), Some(flag::ROMAJI), "/");
		assert_eq!(word_type('·'), Some(flag::ROMAJI), "·"); // U+00B7 Middle Dot
		assert_eq!(word_type('あ'), Some(flag::JAPANESE), "あ");
		assert_eq!(word_type('ア'), Some(flag::JAPANESE), "ア");
		assert_eq!(word_type('０'), Some(flag::JAPANESE), "０");
		assert_eq!(word_type('ー'), Some(flag::JAPANESE), "ー"); // long vowel mark
		assert_eq!(word_type('－'), Some(flag::JAPANESE), "－"); // fullwidth hyphen
		assert_eq!(word_type('・'), Some(flag::JAPANESE), "・");
		assert_eq!(word_type('日'), Some(flag::JAPANESE), "日");
	}

	#[bench]
	fn bench_get_flags(b: &mut Bencher) {
		b.iter(|| get_flags(test::black_box('日')));
	}

	#[bench]
	fn bench_get_none_flags(b: &mut Bencher) {
		b.iter(|| get_flags(test::black_box('\0')));
	}
}
