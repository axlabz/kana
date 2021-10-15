use syn::{
	ext::IdentExt,
	parse::{discouraged::Speculative, Parse},
	Ident, LitChar, LitStr, Token, TypePath,
};

/// List of character mapping expressions used by [`charinfo!`].
pub struct CharMatches(pub TypePath, pub Vec<CharMatch>, pub Option<CharFlag>);

/// Represents a single matching expression from [`charinfo!`].
pub struct CharMatch {
	pub ranges: Vec<CharRange>,
	pub flags: Vec<CharFlag>,
}

/// A single flag for a character matching expression from [`charinfo!`].
///
/// Flags are either one or two identifiers (e.g. `Flag` or `FlagType::Flag`).
pub struct CharFlag(pub String, pub String);

/// Single range of characters from a [`charinfo!`] matching expression.
pub struct CharRange {
	/// Start character of the range.
	pub start: char,

	/// End character of the range (exclusive).
	pub end: char,

	pub span: proc_macro2::Span,
}

impl Parse for CharMatches {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		// resulting type for the macro
		let type_path = input.parse::<TypePath>()?;
		input.parse::<Token![,]>()?;

		let mut matches: Vec<CharMatch> = Vec::new();
		let mut catch_all: Option<CharFlag> = None;
		loop {
			if input.peek(Token![*]) {
				let pos = input.fork();
				input.parse::<Token![*]>()?;
				input.parse::<Token![=>]>()?;
				if catch_all.is_some() {
					return Err(pos.error("only one catch-all rule is allowed"));
				}
				catch_all = Some(input.parse()?);
			} else {
				matches.push(input.parse()?);
			}
			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
				if input.is_empty() {
					break;
				}
			} else {
				break;
			}
		}

		if !input.is_empty() {
			Err(input.error("expected either a comma or end of input"))
		} else {
			Ok(CharMatches(type_path, matches, catch_all))
		}
	}
}

impl Parse for CharMatch {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut ranges: Vec<CharRange> = Vec::new();
		loop {
			match parse_char(input)? {
				CharRangeOrList::Single(chr) => ranges.push(chr),
				CharRangeOrList::List(mut vec) => ranges.append(&mut vec),
			}

			if input.peek(Token![|]) {
				input.parse::<Token![|]>()?;
			} else {
				break;
			}
		}

		input.parse::<Token![=>]>()?;

		let mut flags: Vec<CharFlag> = Vec::new();
		loop {
			flags.push(input.parse::<CharFlag>()?);
			if input.peek(Token![|]) {
				input.parse::<Token![|]>()?;
			} else {
				break;
			}
		}

		Ok(CharMatch { ranges, flags })
	}
}

impl Parse for CharFlag {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let str1 = input.call(Ident::parse)?.unraw().to_string();
		let str2 = if input.peek(Token![::]) {
			input.parse::<Token![::]>()?;
			input.call(Ident::parse)?.unraw().to_string()
		} else {
			String::new()
		};
		Ok(CharFlag(str1, str2))
	}
}

/// Represent either a single [`CharRange`] or a list. Used just as the return
/// of [`CharRangeList::parse_char`].
enum CharRangeOrList {
	Single(CharRange),
	List(Vec<CharRange>),
}

/// Parses a single character range of a [charinfo!] matching expression.
fn parse_char(input: syn::parse::ParseStream) -> syn::Result<CharRangeOrList> {
	// we use exclusive ranges internally, so we need to get the next character
	// after the range end
	fn next_char(chr: char) -> char {
		unsafe { char::from_u32_unchecked((chr as u32) + 1) }
	}

	let fork = input.fork();
	if let Ok(chr) = fork.parse::<LitChar>() {
		input.advance_to(&fork);

		let (is_range, inclusive) = if input.peek(Token![..=]) {
			input.parse::<Token![..=]>()?;
			(true, true)
		} else if input.peek(Token![..]) {
			input.parse::<Token![..]>()?;
			(true, false)
		} else {
			(false, false)
		};

		return if is_range {
			let sta_pos = chr.span();
			let chr = chr.value();
			let pos = input.fork();
			let end = input.parse::<LitChar>()?;
			let end_pos = end.span();
			let end = end.value();
			if end < chr {
				return Err(pos.error("invalid character range (end is before the start)"));
			} else if inclusive && end == chr {
				return Err(pos.error("empty character range is not valid"));
			}

			let end = if inclusive { next_char(end) } else { end };

			Ok(CharRangeOrList::Single(CharRange {
				start: chr,
				end: end,
				span: sta_pos.join(end_pos).unwrap_or(sta_pos),
			}))
		} else {
			let pos = chr.span();
			let chr = chr.value();
			Ok(CharRangeOrList::Single(CharRange {
				start: chr,
				end: next_char(chr),
				span: pos,
			}))
		};
	}

	let fork = input.fork();
	if let Ok(str) = fork.parse::<LitStr>() {
		let pos = str.span();
		input.advance_to(&fork);

		return Ok(CharRangeOrList::List(
			str.value()
				.chars()
				.map(|c| CharRange {
					start: c,
					end: next_char(c),
					span: pos,
				})
				.collect(),
		));
	}

	Err(input
		.error("expected a character range expression (either a char, char range, or str literal)"))
}
