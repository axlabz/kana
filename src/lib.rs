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

#[cfg(test)]
mod tests {
	use crate::get_flags;

	use test::Bencher;

	#[bench]
	fn bench_get_flags(b: &mut Bencher) {
		b.iter(|| get_flags('日'));
	}
}
