use std::{
	fmt::Debug,
	ops::{BitAnd, BitOr, BitOrAssign},
};

/// Separator for flag names in string representations.
pub const SEPARATOR: &'static str = "+";

/// Set of bitwise character flags returned by [`get_flags`](crate::chars::get_flags).
///
/// This cannot be created directly. Constants for the individual flag values
/// are defined in [`flag`]. Those can be combined with the `|` operator.
///
/// See the [`flag`] namespace and individual constant documentation for more
/// information on the available flags.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Flags(u32);

impl Flags {
	/// Numeric value for this flag.
	pub const fn bits(self) -> u32 {
		self.0
	}

	/// Combine Flag values. This is equivalent to the `|` operator but is
	/// available in constant expressions.
	pub const fn and(self, other: Flags) -> Flags {
		Flags(self.0 | other.0)
	}
}

impl Default for Flags {
	fn default() -> Self {
		flag::NONE
	}
}

//----------------------------------------------------------------------------//
// Flag definitions
//----------------------------------------------------------------------------//

const ALL_FLAGS: [(Flags, &'static str); 19] = [
	(flag::NONE, "NONE"),
	(flag::JAPANESE, "JAPANESE"),
	(flag::ROMAJI, "ROMAJI"),
	(flag::SPACE, "SPACE"),
	(flag::HIRAGANA, "HIRAGANA"),
	(flag::KATAKANA, "KATAKANA"),
	(flag::KANA, "KANA"),
	(flag::KANJI, "KANJI"),
	(flag::ROMAN, "ROMAN"),
	(flag::PUNCTUATION, "PUNCTUATION"),
	(flag::SYMBOL, "SYMBOL"),
	(flag::WORD, "WORD"),
	(flag::FULLWIDTH, "FULLWIDTH"),
	(flag::HALFWIDTH, "HALFWIDTH"),
	(flag::SMALL, "SMALL"),
	(flag::NUMBER, "NUMBER"),
	(flag::RARE, "RARE"),
	(flag::RADICAL, "RADICAL"),
	(flag::LINEBREAK, "LINEBREAK"),
];

/// Contains all valid [`Flags`] constants. A combination of these is returned
/// by the [`flags`](crate::chars::flags) function.
///
/// ## Overall categories
///
/// Broad categories that include all mapped characters. Those can be useful
/// to segment portions of text and isolate Japanese and Romaji text in a
/// larger context.
///
/// - [`JAPANESE`]
/// - [`ROMAJI`]
/// - [`SPACE`]
///
/// ## Word characters
///
/// Letter, digits, and mid-word marks. The combination of these can be used
/// to extract possible words in a text.
///
/// - [`HIRAGANA`]
/// - [`KATAKANA`]
/// - [`KANA`]
/// - [`KANJI`]
/// - [`ROMAN`]
///
/// ## Punctuation and symbols
///
/// Non-word text characters. These are usually relevant for text segmentation
/// (punctuation in particular).
///
/// - [`PUNCTUATION`]
/// - [`SYMBOL`]
///
/// ## Extra flags
///
/// These provide additional information about characters across all categories
/// that can be relevant in specific contexts.
///
/// - [`WORD`]
/// - [`FULLWIDTH`]
/// - [`HALFWIDTH`]
/// - [`SMALL`]
/// - [`RARE`]
/// - [`NUMBER`]
/// - [`RADICAL`]
/// - [`LINEBREAK`]
///
pub mod flag {
	use super::Flags;

	/// Default zero value for [`Flags`].
	pub const NONE: Flags = Flags(0);

	//----[ Overall categories ]----------------------------------------------//

	/// Includes characters specific to Japanese. Despite its name, a lot of
	/// the characters are general CJK character, not necessarily being used
	/// in Japanese.
	///
	/// ## Notes
	///
	/// - This includes the [`KANJI`] range, not all of which is specific
	/// to Japanese or even mapped to existing characters.
	///
	/// - The _Ideographic Space_ (`U+3000`) commonly used in Japanese text
	/// is mapped as [`SPACE`] instead.
	pub const JAPANESE: Flags = Flags(1 << 0);

	/// Includes all ASCII and extended (e.g. `āīūēō` and `âîûêô`) characters
	/// that are used in the romanized transliterations.
	///
	/// This includes all characters, including ASCII punctuation and symbols
	/// that have [`JAPANESE`] mappings.
	pub const ROMAJI: Flags = Flags(1 << 1);

	/// Space separator characters, including line breaks.
	///
	/// This includes the entire `Zs` Unicode category, common ASCII space
	/// separators (such as tabs), and [`LINEBREAK`].
	pub const SPACE: Flags = Flags(1 << 2);

	//----[ Word characters ]-------------------------------------------------//

	/// Hiragana characters. Note that some common kana related characters are
	/// flagged as [`KANA`] instead (e.g. the long vowel mark).
	///
	/// See also: [`SMALL`], [`RARE`].
	pub const HIRAGANA: Flags = Flags(1 << 3);

	/// Katakana characters. Note that some common kana related characters are
	/// flagged as [`KANA`] instead (e.g. the long vowel mark).
	///
	/// See also: [`SMALL`], [`RARE`], [`HALFWIDTH`].
	pub const KATAKANA: Flags = Flags(1 << 4);

	/// Includes kana related characters that are not strictly only hiragana
	/// or katakana (even if they fall on the respective Unicode ranges).
	///
	/// Examples of characters with this flag are mid-word punctuation (e.g.
	/// the long vowel mark), the voiced sound marks (both combining and
	/// non-combining), and some [`RARE`] ones (e.g. kana iteration marks).
	///
	/// ## Notes
	///
	/// - For the purposes of text segmentation, those should be considered as
	/// part of a kana word.
	pub const KANA: Flags = Flags(1 << 5);

	/// Kanji characters.
	///
	/// Note that this includes the entire Unicode range for kanji, not all of
	/// which is specific to Japanese or even mapped to existing characters.
	pub const KANJI: Flags = Flags(1 << 6);

	/// Roman letters and digits.
	///
	/// This includes both ASCII and fullwidth characters. Those are flagged
	/// with [`ROMAJI`] and [`JAPANESE`]+[`FULLWIDTH`] flags respectively.
	pub const ROMAN: Flags = Flags(1 << 7);

	//----[ Punctuation and symbols ]-----------------------------------------//

	/// Punctuation characters. Includes both [`JAPANESE`] and [`ROMAJI`]
	/// characters. The [`JAPANESE`] punctuation includes the [`FULLWIDTH`]
	/// equivalents for ASCII.
	///
	/// This also includes mid-word punctuation (e.g. `・` and `ー`). Those are
	/// flagged as [`WORD`].
	///
	/// The purpose of this flag is to mark characters that are particularly
	/// relevant to text segmentation and word processing (for example, when
	/// compared to plain [`SYMBOL`] characters).
	pub const PUNCTUATION: Flags = Flags(1 << 8);

	/// Non-word textual symbols. Note that [`PUNCTUATION`] is not included in
	/// this.
	pub const SYMBOL: Flags = Flags(1 << 9);

	//----[ Extra flags ]-----------------------------------------------------//

	/// Flags any character that is part of a word. Includes both [`ROMAJI`]
	/// and [`JAPANESE`].
	///
	/// The purpose of this flag is to be used for simple word segmentation.
	///
	/// Includes characters from [`HIRAGANA`], [`KATAKANA`], [`KANA`], [`KANJI`],
	/// and [`ROMAN`].
	///
	/// Some characters in this also include the [`PUNCTUATION`] flag. Those
	/// indicate mid-word punctuation characters such as `・`, `－` and their
	/// romaji equivalents (including `'`).
	pub const WORD: Flags = Flags(1 << 10);

	/// Fullwidth variants of ASCII characters. This includes characters in
	/// the [`ROMAN`], [`PUNCTUATION`], and [`SYMBOL`] ranges.
	///
	/// See also [`NUMBER`].
	pub const FULLWIDTH: Flags = Flags(1 << 11);

	/// Halfwidth [`KATAKANA`] and [`PUNCTUATION`] characters.
	pub const HALFWIDTH: Flags = Flags(1 << 12);

	/// Small [`KATAKANA`] and [`HIRAGANA`] characters.
	///
	/// Note that this includes [`RARE`] small characters such as the Ainu
	/// katakana extensions.
	pub const SMALL: Flags = Flags(1 << 13);

	/// Numeric characters from [`ROMAJI`] and [`JAPANESE`].
	///
	/// Note that this includes the numeral [`KANJI`]. To target decimal digits
	/// specifically, the [`ROMAJI`] and [`FULLWIDTH`] flags can be used with
	/// this one.
	pub const NUMBER: Flags = Flags(1 << 14);

	/// Rare, unusual, and archaic kana characters.
	///
	/// The purpose of this is to flag kana characters that can usually be
	/// mapped to more usual combinations (e.g. to normalize words for lookup).
	pub const RARE: Flags = Flags(1 << 15);

	/// Kanji radical [`SYMBOL`] characters.
	pub const RADICAL: Flags = Flags(1 << 16);

	/// Any linebreak character. All of these are included in [`SPACE`].
	///
	/// Includes:
	/// - `U+000A` <End of Line> (EOL, LF, NL)
	/// - `U+000B` <Line Tabulation> (VT)
	/// - `U+000C` <Form Feed> (FF)
	/// - `U+000D` <Carriage Return> (CR)
	/// - `U+0085` <Next Line> (NEL)
	/// - `U+2028` Line Separator
	/// - `U+2029` Paragraph Separator
	pub const LINEBREAK: Flags = Flags(1 << 17);
}

//----------------------------------------------------------------------------//
// Operators
//----------------------------------------------------------------------------//

impl BitOr for Flags {
	type Output = Flags;

	fn bitor(self, rhs: Self) -> Self::Output {
		Flags(self.0 | rhs.0)
	}
}

impl BitOrAssign for Flags {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}

impl BitAnd for Flags {
	type Output = bool;

	fn bitand(self, rhs: Self) -> Self::Output {
		self.0 & rhs.0 == rhs.0
	}
}

//----------------------------------------------------------------------------//
// Parsing
//----------------------------------------------------------------------------//

#[derive(Debug, Clone)]
pub enum FlagParseError {
	Invalid(String),
	Empty,
}

impl std::fmt::Display for FlagParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FlagParseError::Invalid(name) => {
				write!(f, "invalid flag: name `{}` is not valid", name)
			}
			FlagParseError::Empty => write!(f, "invalid flag: empty string"),
		}
	}
}

impl std::str::FromStr for Flags {
	type Err = FlagParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.trim().len() == 0 {
			return Err(FlagParseError::Empty);
		}
		let mut result = flag::NONE;
		for it in s.split(SEPARATOR) {
			let mut valid = false;
			for &(flag, name) in ALL_FLAGS.iter() {
				if name == it {
					result = result | flag;
					valid = true;
					break;
				}
			}
			if !valid {
				return Err(FlagParseError::Invalid(it.to_string()));
			}
		}
		Ok(result)
	}
}

//----------------------------------------------------------------------------//
// Debug & Display
//----------------------------------------------------------------------------//

impl std::fmt::Display for Flags {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let &Flags(mut value) = self;
		if value == 0 {
			write!(f, "NONE")?;
		} else {
			let mut sep = "";
			for &(Flags(flag), name) in ALL_FLAGS.iter() {
				if flag & value != 0 {
					value = value & !flag;
					write!(f, "{}{}", sep, name)?;
					sep = SEPARATOR;
				}
			}
			if value != 0 {
				write!(f, "{}{}", sep, value)?;
			}
		}

		Ok(())
	}
}

impl std::fmt::Debug for Flags {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self, f)
	}
}

//----------------------------------------------------------------------------//
// Tests
//----------------------------------------------------------------------------//

#[cfg(test)]
mod tests {
	use super::*;

	/// Test that flags support the bitwise OR operator for combining and the
	/// bitwise AND operator for testing.
	#[test]
	fn support_bit_ops() {
		let flag: Flags = flag::HIRAGANA | flag::KATAKANA;
		assert!(flag & flag::HIRAGANA);
		assert!(flag & flag::KATAKANA);
		assert!(!(flag & flag::JAPANESE));
		assert!(!(flag & (flag::KATAKANA | flag::JAPANESE)));
	}

	/// Test the `and()` function for combining flags in `const` values.
	#[test]
	fn and() {
		const FLAG: Flags = flag::HIRAGANA.and(flag::KATAKANA);
		assert!(FLAG & flag::HIRAGANA);
		assert!(FLAG & flag::KATAKANA);
		assert!(!(FLAG & flag::JAPANESE));
	}

	/// Make sure that the defined flag constants are unique and don't overlap.
	#[test]
	fn are_unique() {
		for i in 0..ALL_FLAGS.len() {
			for j in i + 1..ALL_FLAGS.len() {
				let l = ALL_FLAGS[i].0;
				let r = ALL_FLAGS[j].0;
				assert_ne!(
					l, r,
					"flag constants should not be equal ([{}] == [{}])",
					i, j
				);

				assert!(
					l.bits() & r.bits() == 0,
					"flag constant bits should not overlap ([{}]:{} & [{}]:{})",
					i,
					l,
					j,
					r,
				)
			}
		}
	}

	/// Check that all flags parse properly.
	#[test]
	fn should_parse() {
		let mut combined = flag::NONE;
		for &(flag, name) in ALL_FLAGS.iter() {
			let parsed: Flags = name.parse().unwrap();
			assert_eq!(parsed, flag);
			combined |= flag;
		}

		let all = ALL_FLAGS
			.iter()
			.map(|&(_, name)| name)
			.collect::<Vec<_>>()
			.join("+");
		let parsed: Flags = all.parse().unwrap();
		assert_eq!(parsed, combined);
	}

	/// Check that all flags can parse their display representation.
	#[test]
	fn should_parse_display() {
		let mut combined = flag::NONE;
		for &(flag, _) in ALL_FLAGS.iter() {
			let text = format!("{}", flag);
			let parsed: Flags = text.parse().unwrap();
			assert_eq!(parsed, flag);
			combined |= flag;
		}

		let text = format!("{}", combined);
		let parsed: Flags = text.parse().unwrap();
		assert_eq!(parsed, combined);
	}
}
