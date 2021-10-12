use std::ops::{BitAnd, BitOr, BitOrAssign};

/// Separator for flag names in string representations.
pub const SEPARATOR: &'static str = "+";

/// Set of bitwise character flags.
///
/// Constants for the individual flag values are defined in [`Flags`].
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Flags(u32);

impl Flags {
	/// Numeric value for this flag.
	pub fn bits(&self) -> u32 {
		self.0
	}
}

/// Contains all valid [`Flag`] constants.
#[allow(non_snake_case)]
pub mod Flag {
	use super::Flags;

	pub const NONE: Flags = Flags(0);
	pub const JAPANESE: Flags = Flags(1 << 0);
	pub const ROMAJI: Flags = Flags(1 << 1);
	pub const SPACE: Flags = Flags(1 << 2);
	pub const HIRAGANA: Flags = Flags(1 << 3);
	pub const KATAKANA: Flags = Flags(1 << 4);
	pub const KANA: Flags = Flags(1 << 5);
	pub const KANJI: Flags = Flags(1 << 6);
	pub const ROMAN: Flags = Flags(1 << 7);
	pub const PUNCTUATION: Flags = Flags(1 << 8);
	pub const SYMBOL: Flags = Flags(1 << 9);
	pub const WORD: Flags = Flags(1 << 10);
	pub const FULLWIDTH: Flags = Flags(1 << 11);
	pub const HALFWIDTH: Flags = Flags(1 << 12);
	pub const SMALL: Flags = Flags(1 << 13);
	pub const NUMBER: Flags = Flags(1 << 14);
	pub const RARE: Flags = Flags(1 << 15);
	pub const RADICAL: Flags = Flags(1 << 16);
}

const ALL_FLAGS: [(Flags, &'static str); 18] = [
	(Flag::NONE, "NONE"),
	(Flag::JAPANESE, "JAPANESE"),
	(Flag::ROMAJI, "ROMAJI"),
	(Flag::SPACE, "SPACE"),
	(Flag::HIRAGANA, "HIRAGANA"),
	(Flag::KATAKANA, "KATAKANA"),
	(Flag::KANA, "KANA"),
	(Flag::KANJI, "KANJI"),
	(Flag::ROMAN, "ROMAN"),
	(Flag::PUNCTUATION, "PUNCTUATION"),
	(Flag::SYMBOL, "SYMBOL"),
	(Flag::WORD, "WORD"),
	(Flag::FULLWIDTH, "FULLWIDTH"),
	(Flag::HALFWIDTH, "HALFWIDTH"),
	(Flag::SMALL, "SMALL"),
	(Flag::NUMBER, "NUMBER"),
	(Flag::RARE, "RARE"),
	(Flag::RADICAL, "RADICAL"),
];

//----------------------------------------------------------------------------//
// OPERATOR
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
		self.0 & rhs.0 != 0
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
		let mut result = Flag::NONE;
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

	#[test]
	fn flags_are_unique() {
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

	#[test]
	fn flags_should_parse() {
		let mut combined = Flag::NONE;
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

	#[test]
	fn flags_should_parse_display() {
		let mut combined = Flag::NONE;
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
