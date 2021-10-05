use proc_macro::TokenStream;
use quote::quote;
use syn::{
	ext::IdentExt,
	parse::{discouraged::Speculative, Parse},
	parse_macro_input, Ident, LitChar, LitStr, Token,
};

struct CharMatches(Vec<CharMatch>);

/// Represents a single matching expression from [`charinfo!`].
struct CharMatch {
	ranges: Vec<CharRange>,
	flags: Vec<CharFlag>,
}

/// A single flag for a character matching expression from [`charinfo!`].
struct CharFlag(String);

/// Single range of characters from a [`charinfo!`] matching expression.
struct CharRange {
	/// Start character of the range.
	start: char,

	/// End character of the range.
	end: char,

	/// Is this range inclusive?
	inclusive: bool,
}

impl Parse for CharMatches {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut matches: Vec<CharMatch> = Vec::new();
		loop {
			matches.push(input.parse()?);
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
			Ok(CharMatches(matches))
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
			let str = input.call(Ident::parse)?.unraw().to_string();
			flags.push(CharFlag(str));
			if input.peek(Token![|]) {
				input.parse::<Token![|]>()?;
			} else {
				break;
			}
		}

		Ok(CharMatch { ranges, flags })
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
	let fork = input.fork();
	if let Ok(chr) = fork.parse::<LitChar>() {
		input.advance_to(&fork);

		let (is_range, inclusive) = if input.peek(Token![..]) {
			input.parse::<Token![..]>()?;
			(true, false)
		} else if input.peek(Token![..=]) {
			input.parse::<Token![..=]>()?;
			(true, true)
		} else {
			(false, false)
		};

		return if is_range {
			let end = input.parse::<LitChar>()?;
			Ok(CharRangeOrList::Single(CharRange {
				start: chr.value(),
				end: end.value(),
				inclusive,
			}))
		} else {
			let chr = chr.value();
			Ok(CharRangeOrList::Single(CharRange {
				start: chr,
				end: chr,
				inclusive: true,
			}))
		};
	}

	let fork = input.fork();
	if let Ok(str) = fork.parse::<LitStr>() {
		input.advance_to(&fork);

		return Ok(CharRangeOrList::List(
			str.value()
				.chars()
				.map(|c| CharRange {
					start: c,
					end: c,
					inclusive: true,
				})
				.collect(),
		));
	}

	Err(input
		.error("expected a character range expression (either a char, char range, or str literal)"))
}

/// Receives a list of character matching expressions and returns a closure
/// function that maps a single `char` to the respective mapping combination.
///
/// Matching expressions map a set of characters to bitflag value. They have
/// the syntax `CHARS => FLAGS`, where:
///
/// - `CHARS` is a bitwise-or combination of character ranges (see below);
/// - `FLAGS` is a bitwise-or combination of one or more identifiers mapping
///   to constant values that can be combined with bitwise-OR.
///
/// Character ranges may overlap freely. The result for any given `char` is
/// the bitwise-OR combination of the flags of all ranges that contain it.
///
/// Character ranges may be specified as:
/// - Single `char` literal;
/// - Bounded range of two `char` literals;
/// - String literal which maps to each char in the string.
///
/// This macro results in a closure `|x: char| -> Option<F>` where `F` is the
/// resulting type of the provided flags.
#[proc_macro]
pub fn charinfo(input: TokenStream) -> TokenStream {
	let _input = parse_macro_input!(input as CharMatches);

	TokenStream::from(quote! {
		|x: char| -> Option<u32> { ::std::option::Option::None }
	})
}
