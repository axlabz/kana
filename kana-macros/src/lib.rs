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
///
/// If a "catch-all" rule with the syntax `* => FLAG` is provided, then instead
/// of returning an `Option`, the generated closure will return the given flag
/// for unmapped characters.
#[proc_macro]
pub fn charinfo(input: TokenStream) -> TokenStream {
	let mut charset = RangeBuilder::new();
	let CharMatches(type_path, input, catch_all) = parse_macro_input!(input as CharMatches);
	for m in input {
		for range in m.ranges {
			for CharFlag(flag1, flag2) in &m.flags {
				let flag = if flag2.len() > 0 {
					format!("{}::{}", flag1, flag2)
				} else {
					flag1.clone()
				};
				charset = charset.add(range.start, range.end, &flag);
			}
		}
	}

	let has_catch_all = catch_all.is_some();

	let to_flags = |flags: Vec<&str>| {
		let flags = flags.into_iter().map(|x| {
			if x.contains("::") {
				let ids = x.split("::").map(|x| format_ident!("{}", x));
				quote! { #( #ids )::* }
			} else {
				let id = format_ident!("{}", x);
				quote! { #id }
			}
		});
		if has_catch_all {
			quote! { #( #flags )|* }
		} else {
			quote! { Some( #( #flags )|* ) }
		}
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

	let catch_all = if let Some(CharFlag(a, b)) = &catch_all {
		let catch_all = if b.len() > 0 {
			let (a, b) = (format_ident!("{}", a), format_ident!("{}", b));
			quote! { #a :: #b }
		} else {
			let id = format_ident!("{}", a);
			quote! { #id }
		};
		quote! { _ => #catch_all }
	} else {
		quote! { _ => ::std::option::Option::None }
	};

	let return_type = if has_catch_all {
		quote! { #type_path }
	} else {
		quote! { ::std::option::Option<#type_path> }
	};

	let tokens = quote! {
		|x: char| -> #return_type {
			match x {
				#( #matches )*
				#catch_all
			}
		}
	};

	tokens.into()
}
