use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod chars;
use chars::{CharFlag, CharMatches};

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
