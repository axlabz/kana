//! Character tables

use kana_macros::charinfo;

/// This namespace contains the flags returned by the [`flags`](super::flags)
/// function.
///
/// The main purpose for these flags is to support text segmentation and
/// classification. This drives the rationale behind the mapping of flags to
/// characters, which will not always correspond to the proper Unicode or
/// linguistic categories.
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
/// to parse words in a text.
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
/// These provide additional information about characters in the other
/// categories that can be relevant in specific contexts.
///
/// - [`FULLWIDTH`]
/// - [`HALFWIDTH`]
/// - [`SMALL`]
/// - [`RARE`]
/// - [`NUMBER`]
/// - [`RADICAL`]
///
#[allow(non_snake_case)]
pub mod Flags {
	/// Zero flag returned for unmapped characters.
	pub const NONE: u32 = 0;

	//----[ Overall categories ]----------------------------------------------//

	/// Includes all Japanese characters and the [`KANJI`] ranges (which
	/// contains characters that are not Japanese).
	///
	/// Note that this does not include the `U+3000` ideographic space that is
	/// commonly used for Japanese text, which is flagged as [`SPACE`].
	pub const JAPANESE: u32 = 1 << 0;

	/// Includes all ASCII and extended (e.g. `āīūēō` and `âîûêô`) characters
	/// that are used in the romanized transliterations.
	///
	/// This includes everything, including ASCII punctuation and symbols that
	/// have [`JAPANESE`] mappings.
	pub const ROMAJI: u32 = 1 << 1;

	/// Space separator characters, including line breaks.
	///
	/// This includes:
	///
	/// - TAB character.
	/// - The entire `Zs` "Space Separator" Unicode category.
	/// - `\r`, and `\n` ASCII line break characters.
	/// - `U+2028` and `U+2029` (Line and Paragraph separators).
	///
	/// In the context of Japanese text segmentation, spaces are rarely used
	/// mid-sentence, so space characters can be used to do top-level
	/// segmentation of the text.
	pub const SPACE: u32 = 1 << 2;

	//----[ Word characters ]-------------------------------------------------//

	/// Hiragana characters.
	///
	/// This also includes [`RARE`] characters in the hiragana range, such
	/// as `ゝ` and `ゞ` iteration marks and the `ゟ` yori digraph.
	///
	/// Note that for the purposes of text segmentation, some common mid-word
	/// characters are flagged as [`KANA`].
	pub const HIRAGANA: u32 = 1 << 3;

	/// Katakana characters.
	///
	/// This includes [`HALFWIDTH`] and [`RARE`] characters in the katakana
	/// range (e.g. `ヽ` and `ヾ` iteration marks, the `ヿ` koto digraph, and
	/// the Ainu small katakana extensions).
	///
	/// Note that for the purposes of text segmentation, some common mid-word
	/// characters are flagged as [`KANA`]. This includes some characters in
	/// the usual katakana range, such as the long vowel mark.
	pub const KATAKANA: u32 = 1 << 4;

	/// Kana characters that are not strictly just hiragana or katakana and are
	/// commonly used in words.
	///
	/// For the purposes of text segmentation, those can be considered as part
	/// of a word.
	///
	/// Examples in this category are the long vowel mark, voiced marks (both
	/// combining and non-combining), and the `・` katakana middle dot.
	pub const KANA: u32 = 1 << 5;

	/// Kanji characters.
	///
	/// Note that this includes the entire unicode range for kanji, so not all
	/// characters are necessarily valid kanji, and not all are Japanese.
	pub const KANJI: u32 = 1 << 6;

	/// Roman letters and digits.
	///
	/// This includes both ASCII and fullwidth characters. Those are flagged
	/// with [`ROMAJI`] and [`JAPANESE`]+[`FULLWIDTH`] flags respectively.
	pub const ROMAN: u32 = 1 << 7;

	//----[ Punctuation and symbols ]-----------------------------------------//

	/// Non-word punctuation symbols. As opposed to [`SYMBOL`], punctuation is
	/// usually relevant to the sentence structure.
	///
	/// This includes [`JAPANESE`] and [`ROMAJI`] punctuation. In [`JAPANESE`],
	/// the fullwidth ASCII punctuation can be identified by [`FULLWIDTH`].
	///
	/// Note that some characters that could be considered punctuation are
	/// actually classified as [`KANA`] because they can occur mid-word. One
	/// example is the `・` katakana middle dot.
	///
	/// See also [`SYMBOL`].
	pub const PUNCTUATION: u32 = 1 << 8;

	/// Non-word symbol characters. This is similar to [`PUNCTUATION`], but
	/// symbols are characters that usually don't affect the sentence structure.
	///
	/// In the context of text segmentation, symbols are not part of words (i.e.
	/// they break words), but are not used to segment sentences.
	pub const SYMBOL: u32 = 1 << 9;

	//----[ Extra flags ]-----------------------------------------------------//

	/// Fullwidth variants of ASCII characters. This includes characters in
	/// the [`ROMAN`], [`PUNCTUATION`], and [`SYMBOL`] ranges.
	///
	/// Combining this with [`NUMBER`] makes it possible to target fullwidth
	/// digit characters.
	pub const FULLWIDTH: u32 = 1 << 10;

	/// Halfwidth [`KATAKANA`] characters.
	pub const HALFWIDTH: u32 = 1 << 11;

	/// Small [`KATAKANA`] and [`HIRAGANA`] characters.
	///
	/// Note that this includes [`RARE`] small characters as the Ainu katakana
	/// extensions.
	pub const SMALL: u32 = 1 << 12;

	/// Numeric characters. This includes [`ROMAJI`] and [`JAPANESE`], including
	/// kanji used for numbers.
	///
	/// To target decimal digits specifically, the [`ROMAJI`] and [`FULLWIDTH`]
	/// flags can be used in combination with this.
	pub const NUMBER: u32 = 1 << 13;

	/// Rare, unusual, and archaic kana characters. Characters with this flag
	/// can be trivially mapped to more common kana variants.
	pub const RARE: u32 = 1 << 14;

	/// Kanji radicals symbols.
	pub const RADICAL: u32 = 1 << 15;
}

/// Returns a set of the flags mapped for the given character. The flags are a
/// bitwise combination of the constants in the [`Flags`] namespace.
///
/// If the given character is not mapped, returns zero (i.e. [`Flags::NONE`]).
///
/// See the [`Flags`] namespace and individual flags for more details on the
/// mappings and rationale behind the flags.
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
