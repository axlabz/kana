//! Character tables

use kana_macros::charinfo;

use crate::{flag, Flags};

// Some common flag combinations:

const KANJI: Flags = flag::JAPANESE.and(flag::WORD).and(flag::KANJI);
const HIRAGANA: Flags = flag::JAPANESE.and(flag::WORD).and(flag::HIRAGANA);
const KATAKANA: Flags = flag::JAPANESE.and(flag::WORD).and(flag::KATAKANA);
const KANA: Flags = flag::JAPANESE.and(flag::WORD).and(flag::KANA);
const JP_SYMBOL: Flags = flag::JAPANESE.and(flag::SYMBOL);
const JP_PUNCTUATION: Flags = flag::JAPANESE.and(flag::PUNCTUATION);

/// Returns a set of the flags mapped for the given character. The flags are a
/// bitwise combination of the constants in the [`Flags`] namespace.
///
/// If the given character is not mapped, returns zero (i.e. [`Flags::NONE`]).
///
/// See the [`Flags`] namespace and individual flags for more details on the
/// mappings and rationale behind the flags.
pub fn get_flags(chr: char) -> Flags {
	let get = charinfo!(

		//--------------------------------------------------------------------//
		// Spaces
		//--------------------------------------------------------------------//

		" \t" => flag::SPACE,

		// space separator Unicode category
		'\u{00A0}' | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' |
		'\u{2003}' | '\u{2004}' | '\u{2005}' | '\u{2006}' | '\u{2007}' |
		'\u{2008}' | '\u{2009}' | '\u{200A}' | '\u{202F}' | '\u{205F}' |
		'\u{3000}' => flag::SPACE,

		// vertical tab and form feed
		"\x0B\x0C" => flag::SPACE | flag::LINEBREAK,

		// line breaks
		"\r\n" | '\u{0085}' | '\u{2028}' | '\u{2029}' => flag::SPACE | flag::LINEBREAK,

		//--------------------------------------------------------------------//
		// ASCII
		//--------------------------------------------------------------------//

		// Letters and digits
		'0'..='9' => flag::ROMAJI | flag::ROMAN | flag::WORD | flag::NUMBER,
		'A'..='Z' => flag::ROMAJI | flag::ROMAN | flag::WORD,
		'a'..='z' => flag::ROMAJI | flag::ROMAN | flag::WORD,

		// Extended vowels:
		"âêîôû" | "āēīōū" => flag::ROMAJI | flag::ROMAN | flag::WORD,
		"ÂÊÎÔÛ" | "ĀĒĪŌŪ" => flag::ROMAJI | flag::ROMAN | flag::WORD,

		// This is a punctuation if you want to parse quotes
		'"' => flag::ROMAJI | flag::PUNCTUATION,

		// Apostrophe is used to disambiguate the romaji
		'\'' => flag::ROMAJI | flag::PUNCTUATION | flag::WORD,

		// Hyphen is used for long vowels
		'-' => flag::ROMAJI | flag::PUNCTUATION | flag::WORD,

		// Solidus is used as equivalent to the kana middle-dot
		'/' => flag::ROMAJI | flag::PUNCTUATION | flag::WORD,

		// U+00B7 - Middle Dot (not the Japanese one)
		'·' => flag::ROMAJI | flag::PUNCTUATION | flag::WORD,

		// Punctuation:
		'!' | ',' | '.' | ':' | ';' | '?' => flag::ROMAJI | flag::PUNCTUATION,

		// Parenthesis and brackets:
		'(' | ')' | '[' | ']' | '{' | '}' => flag::ROMAJI | flag::PUNCTUATION,

		// Symbols:
		'#' | '$' | '%' | '&' | '*' | '+' | '<' | '='  => flag::ROMAJI | flag::SYMBOL,
		'>' | '@' | '^' | '_' | '`' | '|' | '~' | '\\' => flag::ROMAJI | flag::SYMBOL,

		// Additional punctuation
		"«»" => flag::ROMAJI | flag::PUNCTUATION,

		// Additional symbols
		"¢£¥¦¬¯₩" => flag::ROMAJI | flag::SYMBOL,

		//--------------------------------------------------------------------//
		// KANJI
		//--------------------------------------------------------------------//

		// For kanji we just use the Unicode ranges
		'\u{3400}'..='\u{4DBF}'   => KANJI, // CJK Unified Ideographs Extension A
		'\u{4E00}'..='\u{9FFF}'   => KANJI, // CJK Unified Ideographs
		'\u{F900}'..='\u{FAFF}'   => KANJI, // CJK Compatibility Ideographs
		'\u{20000}'..='\u{2A6DF}' => KANJI, // CJK Unified Ideographs Extension B
		'\u{2A700}'..='\u{2B73F}' => KANJI, // CJK Unified Ideographs Extension C
		'\u{2B740}'..='\u{2B81F}' => KANJI, // CJK Unified Ideographs Extension D
		'\u{2B820}'..='\u{2CEAF}' => KANJI, // CJK Unified Ideographs Extension E
		'\u{2CEB0}'..='\u{2EBEF}' => KANJI, // CJK Unified Ideographs Extension F

		// Numeric kanji
		"零一二三四五六七八九十百千万億兆" => flag::NUMBER,

		// We consider this a kanji for all intents and purposes:
		// - U+3005 々 Ideographic Iteration Mark
		'々' => KANJI,

		//--------------------------------------------------------------------//
		// HIRAGANA
		//--------------------------------------------------------------------//

		// Hiragana range:
		// - U+3041 ぁ Hiragana Letter Small A, to
		// - U+3096 ゖ Hiragana Letter Small Ke
		'ぁ'..='ゖ' => HIRAGANA,

		// Small characters
		"ぁぃぅぇぉっゃゅょ" => flag::SMALL,
		"ゎゕゖ" => flag::SMALL | flag::RARE,

		// Rare characters
		"ゐゑ" => flag::RARE,

		// Additional archaic characters
		// - U+309F ゟ Hiragana Digraph Yori
		// - U+1B001 𛀁 Hiragana Letter Archaic Ye
		'\u{309F}' | '\u{1B001}' => HIRAGANA | flag::RARE,

		// - U+1B150 Hiragana Letter Small Wi
		// - U+1B151 Hiragana Letter Small We
		// - U+1B152 Hiragana Letter Small Wo
		'\u{1B150}' | '\u{1B151}' | '\u{1B152}' => HIRAGANA | flag::RARE | flag::SMALL,

		//--------------------------------------------------------------------//
		// KATAKANA
		//--------------------------------------------------------------------//

		// U+30A0 ゠ Katakana-Hiragana Double Hyphen
		'\u{30A0}' => JP_SYMBOL,

		// Katakana range:
		// - U+30A1 ァ Katakana Letter Small A
		// - U+30FA ヺ Katakana Letter Vo
		'ァ'..='ヺ' => KATAKANA,

		// Small characters
		"ァィゥェォッャュョ" => flag::SMALL,
		"ヮヵヶ" => flag::SMALL | flag::RARE,

		// Rare characters
		"ヰヱヸヹヺヷ" => flag::RARE,

		// Additional archaic characters
		// - U+30FF ヿ Katakana Digraph Koto
		// U+1B000 𛀀 Katakana Letter Archaic E
		"ヿ𛀀" => KATAKANA | flag::RARE,

		// U+1B164 𛅤 Katakana Letter Small Wi
		// U+1B165 𛅥 Katakana Letter Small We
		// U+1B166 𛅦 Katakana Letter Small Wo
		// U+1B167 𛅧 Katakana Letter Small N
		'\u{1B164}'..='\u{1B167}' => KATAKANA | flag::RARE | flag::SMALL,

		// U+31F0 ㇰ Katakana Letter Small Ku
		// U+31F1 ㇱ Katakana Letter Small Si
		// U+31F2 ㇲ Katakana Letter Small Su
		// U+31F3 ㇳ Katakana Letter Small To
		// U+31F4 ㇴ Katakana Letter Small Nu
		// U+31F5 ㇵ Katakana Letter Small Ha
		// U+31F6 ㇶ Katakana Letter Small Hi
		// U+31F7 ㇷ Katakana Letter Small Hu
		// U+31F8 ㇸ Katakana Letter Small He
		// U+31F9 ㇹ Katakana Letter Small Ho
		// U+31FA ㇺ Katakana Letter Small Mu
		// U+31FB ㇻ Katakana Letter Small Ra
		// U+31FC ㇼ Katakana Letter Small Ri
		// U+31FD ㇽ Katakana Letter Small Ru
		// U+31FE ㇾ Katakana Letter Small Re
		// U+31FF ㇿ Katakana Letter Small Ro
		"ㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ" => KATAKANA | flag::SMALL | flag::RARE,

		// Symbol range:
		// - U+32D0 ㋐ Circled Katakana A
		// - U+32FE ㋾ Circled Katakana Wo
		// - U+1F202 🈂 Squared Katakana Sa
		// - U+1F213 🈓 Squared Katakana De
		'\u{32D0}'..='\u{32FE}' | '\u{1F202}' | '\u{1F213}' => JP_SYMBOL,

		// Halfwidth katakana range:
		// - U+FF66 ｦ Halfwidth Katakana Letter Wo
		// - Skip: U+FF70 ｰ Halfwidth Katakana-Hiragana Prolonged Sound Mark
		// - U+FF9D ﾝ Halfwidth Katakana Letter N
		'\u{FF66}'..='\u{FF6F}' | '\u{FF71}'..='\u{FF9D}' => KATAKANA | flag::HALFWIDTH,

		// Halfwidth small range:
		// - U+FF67 ｧ Halfwidth Katakana Letter Small A
		// - U+FF6F ｯ Halfwidth Katakana Letter Small Tu
		'\u{FF67}'..='\u{FF6F}' => flag::SMALL,

		//--------------------------------------------------------------------//
		// KANA
		//--------------------------------------------------------------------//

		// U+3099 Combining Katakana-Hiragana Voiced Sound Mark
		// U+309A Combining Katakana-Hiragana Semi-Voiced Sound Mark
		'\u{3099}' | '\u{309A}' => KANA,

		// U+309B ゛ Katakana-Hiragana Voiced Sound Mark
		// U+309C ゜ Katakana-Hiragana Semi-Voiced Sound Mark
		// U+309D ゝ Hiragana Iteration Mark
		// U+309E ゞ Hiragana Voiced Iteration Mark
		'\u{309B}' | '\u{309C}' | '\u{309D}' | '\u{309E}' => KANA | flag::PUNCTUATION | flag::RARE,

		// U+30FC ー Katakana-Hiragana Prolonged Sound Mark
		'\u{30FC}' => KANA | flag::PUNCTUATION,

		// U+30FB ・ Katakana Middle Dot
		// U+30FC ー Katakana-Hiragana Prolonged Sound Mark
		'\u{30FB}' | '\u{30FC}' => KANA | flag::PUNCTUATION,

		// U+30FD ヽ Katakana Iteration Mark
		// U+30FE ヾ Katakana Voiced Iteration Mark
		'\u{30FD}' | '\u{30FE}' => KANA | flag::PUNCTUATION | flag::RARE,

		// U+FF65 ･ Halfwidth Katakana Middle Dot
		// U+FF70 ｰ Halfwidth Katakana-Hiragana Prolonged Sound Mark
		'\u{FF65}' | '\u{FF70}' => KANA | flag::PUNCTUATION | flag::HALFWIDTH,

		// U+FF9E ﾞ Halfwidth Katakana Voiced Sound Mark
		// U+FF9F ﾟ Halfwidth Katakana Semi-Voiced Sound Mark
		'\u{FF9E}' | '\u{FF9F}' => KANA | flag::PUNCTUATION | flag::HALFWIDTH | flag::RARE,

		// U+303C 〼 Masu Mark
		'〼' => KANA | flag::RARE,

		//--------------------------------------------------------------------//
		// FULLWIDTH
		//--------------------------------------------------------------------//

		"！＂（），．：；？［］｛｝｟｠" => flag::FULLWIDTH | JP_PUNCTUATION,

		"＃＄％＆＇＊＋－／＜＝＞＠＼＾＿｀｜～￠￡￢￣￤￥￦" => flag::FULLWIDTH | JP_SYMBOL,

		'Ａ'..='Ｚ' | 'ａ'..='ｚ' => flag::FULLWIDTH | flag::JAPANESE | flag::ROMAN | flag::WORD,

		'０'..='９' => flag::FULLWIDTH | flag::JAPANESE | flag::ROMAN | flag::NUMBER | flag::WORD,

		//--------------------------------------------------------------------//
		// JAPANESE PUNCTUATION
		//--------------------------------------------------------------------//

		// Halfwidth:
		// - U+FF61 ｡ Halfwidth Ideographic Full Stop
		// - U+FF62 ｢ Halfwidth Left Corner Bracket
		// - U+FF63 ｣ Halfwidth Right Corner Bracket
		// - U+FF64 ､ Halfwidth Ideographic Comma
		"｡｢｣､" => flag::HALFWIDTH | JP_PUNCTUATION,

		// U+3001	、	Ideographic Comma
		// U+3002	。	Ideographic Full Stop
		// U+3008	〈	Left Angle Bracket
		// U+3009	〉	Right Angle Bracket
		// U+300A	《	Left Double Angle Bracket
		// U+300B	》	Right Double Angle Bracket
		// U+300C	「	Left Corner Bracket
		// U+300D	」	Right Corner Bracket
		// U+300E	『	Left White Corner Bracket
		// U+300F	』	Right White Corner Bracket
		// U+3010	【	Left Black Lenticular Bracket
		// U+3011	】	Right Black Lenticular Bracket
		// U+3014	〔	Left Tortoise Shell Bracket
		// U+3015	〕	Right Tortoise Shell Bracket
		// U+3016	〖	Left White Lenticular Bracket
		// U+3017	〗	Right White Lenticular Bracket
		// U+3018	〘	Left White Tortoise Shell Bracket
		// U+3019	〙	Right White Tortoise Shell Bracket
		// U+301A	〚	Left White Square Bracket
		// U+301B	〛	Right White Square Bracket
		// U+301D	〝	Reversed Double Prime Quotation Mark
		// U+301E	〞	Double Prime Quotation Mark
		// U+301F	〟	Low Double Prime Quotation Mark
		"、。〈〉《》「」『』【】〔〕〖〗〘〙〚〛〝〞〟" => JP_PUNCTUATION,

		//--------------------------------------------------------------------//
		// JAPANESE SYMBOLS
		//--------------------------------------------------------------------//

		// Halfwidth:
		// U+FFE8 ￨ Halfwidth Forms Light Vertical
		// U+FFE9 ￩ Halfwidth Leftwards Arrow
		// U+FFEA ￪ Halfwidth Upwards Arrow
		// U+FFEB ￫ Halfwidth Rightwards Arrow
		// U+FFEC ￬ Halfwidth Downwards Arrow
		// U+FFED ￭ Halfwidth Black Square
		// U+FFEE ￮ Halfwidth White Circle
		"￨￩￪￬￫￭￮" =>  flag::HALFWIDTH | JP_SYMBOL,

		// CJK Symbols and Punctuation

		// Enclosed CJK Letters and Months
		'\u{3200}'..'\u{321F}' | '\u{3220}'..='\u{32FF}' => JP_SYMBOL,

		// CJK Compatibility
		'\u{3300}'..='\u{33FF}' => JP_SYMBOL,

		// numeric
		"〇㈠㈡㈢㈣㈤㈥㈦㈧㈨㈩" => flag::NUMBER,
		"㉈㉉㉊㉋㉌㉍㉎㉏" => flag::NUMBER,
		"㉑㉒㉓㉔㉕㉖㉗㉘㉙㉚㉛㉜㉝㉞㉟" => flag::NUMBER,
		"㊀㊁㊂㊃㊄㊅㊆㊇㊈㊉" => flag::NUMBER,
		"㊱㊲㊳㊴㊵㊶㊷㊸㊹㊺㊻㊼㊽㊾㊿" => flag::NUMBER,

		// CJK Symbols and Punctuation:

		"〃〄〆〇〒〓〜〠〡〢〣〤〥〦〧〨〩〰〱〲〳〴〵〶〷〸〹〺〻〽〾〿" => JP_SYMBOL,

		// U+302E 〮 Hangul Single Dot Tone Mark
		// U+302F 〯 Hangul Double Dot Tone Mark
		// U+302A ◌〪 Ideographic Level Tone Mark
		// U+302B ◌〫 Ideographic Rising Tone Mark
		// U+302C ◌〬 Ideographic Departing Tone Mark
		// U+302D ◌〭 Ideographic Entering Tone Mark
		'\u{302E}' | '\u{302F}' | '\u{302A}'..='\u{302D}' => JP_SYMBOL,

		// Radicals

		// CJK Radicals Supplement
		'\u{2E80}'..'\u{2E9A}' | '\u{2E9B}'..='\u{2EF3}' => JP_SYMBOL | flag::RADICAL,

		// Kangxi Radicals, U+2F00 - U+2FDF
		'\u{2F00}'..='\u{2FD5}' => JP_SYMBOL | flag::RADICAL,

		//--------------------------------------------------------------------//

		// default for any unmatched char
		* => flag::NONE,
	);

	get(chr)
}
