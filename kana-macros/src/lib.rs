use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod chars;
use chars::{CharFlag, CharMatches};

mod ranges;
use ranges::RangeBuilder;

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
	let mut charset = RangeBuilder::new();
	let CharMatches(input) = parse_macro_input!(input as CharMatches);
	for m in input {
		for range in m.ranges {
			for CharFlag(flag) in &m.flags {
				charset = charset.add(range.start, range.end, flag);
			}
		}
	}

	let to_flags = |flags: Vec<&str>| {
		let flags = flags.into_iter().map(|x| format_ident!("{}", x));
		quote! { Some( #( #flags )|* ) }
	};

	let matches = charset.ranges().map(|(start, end, flags)| {
		let flags = to_flags(flags);

		// go back to an inclusive range
		let end = (end as u32) - 1;
		let end = unsafe { char::from_u32_unchecked(end) };
		if start == end {
			quote! { #start => #flags, }
		} else {
			quote! { #start ..= #end => #flags, }
		}
	});

	let tokens = quote! {
		|x: char| -> Option<u32> {
			match x {
				#( #matches )*
				_ => None
			}
		}
	};

	tokens.into()
}
