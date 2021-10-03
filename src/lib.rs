//! Kana manipulation library.
//!
//! Provides conversion between hiragana, katakana, and romaji, as well as
//! helper functions related to Japanese characters.

/// Sample function.
pub fn answer() -> u64 {
	42
}

// TODO: test combining marks and spaces

#[cfg(test)]
mod tests {
	use crate::answer;

	#[test]
	fn it_works() {
		let result = answer();
		assert_eq!(result, 42);
	}
}
