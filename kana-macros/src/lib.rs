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
/// The first argument is the type of the character flags, followed by a comma
/// and a list of matching expressions.
///
/// Matching expressions map a set of characters to bitflag value. They have
/// the syntax `CHARS => FLAGS`, where:
///
/// - `CHARS` is a bitwise-or combination of character ranges (see below);
/// - `FLAGS` is a bitwise-or combination of one or more identifiers mapping
/// to constant values that can be combined with bitwise-OR.
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
	// Parse the macro input.
	let CharMatches(type_path, input, catch_all) = parse_macro_input!(input as CharMatches);

	//----[ Compile ranges ]--------------------------------------------------//

	// Compile the parsed ranges into a sequence of non-overlapping ranges and
	// their combined flags.
	let mut charset = RangeBuilder::new();
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

	// if there is a catch-all rule, we return it instead of `None`
	let has_catch_all = catch_all.is_some();

	// return value for an unmapped character
	let catch_all = if let Some(CharFlag(a, b)) = &catch_all {
		if b.len() > 0 {
			let (a, b) = (format_ident!("{}", a), format_ident!("{}", b));
			quote! { #a :: #b }
		} else {
			let id = format_ident!("{}", a);
			quote! { #id }
		}
	} else {
		quote! { ::std::option::Option::None }
	};

	// Generate the code for a set of flags. Each flag is converted to either
	// a plain identifier or a `X::Y` compound. Flags are then combined with
	// the bitwise-or `|` operator.
	//
	// If we don't have a catch-all rule, this will wrap the flag in `Some`,
	// unless `raw` is true.
	let to_flags = |flags: Vec<&str>, raw: bool| {
		let flags = flags.into_iter().map(|x| {
			if x.contains("::") {
				let ids = x.split("::").map(|x| format_ident!("{}", x));
				quote! { #( #ids )::* }
			} else {
				let id = format_ident!("{}", x);
				quote! { #id }
			}
		});
		let flags = quote! { #( #flags )|* };
		if has_catch_all || raw {
			flags
		} else {
			quote! { ::std::option::Option::Some(#flags) }
		}
	};

	// the actual return type for the function
	let return_type = if has_catch_all {
		quote! { #type_path }
	} else {
		quote! { ::std::option::Option<#type_path> }
	};

	// For characters codes below this we do a direct lookup in a static array.
	//
	// This will generate a static array of this size that is initialized with
	// the flags for the respective character.
	const MAX_LOOKUP: char = '\u{FFFF}';

	let ranges = charset.ranges().collect::<Vec<_>>();

	// filter the ranges for which we want to do a direct lookup
	let lookup = ranges
		.iter()
		.filter(|(s, _, _)| s < &MAX_LOOKUP)
		.map(|(s, e, f)| {
			// limit the range to the max position
			if e <= &MAX_LOOKUP {
				(*s, *e, f.clone())
			} else {
				(*s, MAX_LOOKUP, f.clone())
			}
		})
		.collect::<Vec<(char, char, Vec<&str>)>>();

	// filter out the direct lookup ranges
	let ranges = ranges
		.into_iter()
		.filter(|&(_, e, _)| e > MAX_LOOKUP)
		.map(|(s, e, f)| {
			if s < MAX_LOOKUP {
				(MAX_LOOKUP, e, f)
			} else {
				(s, e, f)
			}
		});

	//----[ Direct lookup ]---------------------------------------------------//

	let max_lookup = lookup.iter().map(|&(_, e, _)| e).max().unwrap_or(0 as char) as usize;

	// To avoid incurring into compilation issues due to a huge array, we
	// instead generate code to set the flag values at runtime.
	let lookup = lookup
		.into_iter()
		.map(|(s, e, f)| {
			let s = s as usize;
			let e = e as usize;
			let f = to_flags(f, false);
			quote! {
				for c in #s..#e {
					out[c] = #f;
				}
			}
		})
		.collect::<Vec<_>>();

	// the direct lookup code
	let lookup = if max_lookup > 0 {
		let count = max_lookup;
		let default = catch_all.clone();

		// note that we need to use a Vec here to avoid a stack overflow if
		// the lookup array is too big
		quote! {
			use ::lazy_static::lazy_static;
			use ::std::vec::Vec;
			lazy_static! {
				static ref LOOKUP: Vec<#return_type> = {
					let mut out: Vec<#return_type> = Vec::new();
					out.reserve_exact(#count);
					out.resize(#count, #default);
					#( #lookup )*
					out
				};
			}

			let code = x as usize;
			if code < #count {
				return LOOKUP[code];
			}
		}
	} else {
		quote! {}
	};

	//----[ Range match ]-----------------------------------------------------//

	// For any remaining ranges, we just generate a `match` expression.
	let ranges = ranges
		.map(|(start, end, flags)| {
			let flags = to_flags(flags, false);
			// exclusive ranges are experimental, convert to inclusive
			let end = unsafe { char::from_u32_unchecked((end as u32) - 1) };
			quote! {
				#start..=#end => #flags,
			}
		})
		.collect::<Vec<_>>();

	let ranges = if ranges.len() > 0 {
		quote! {
			match x {
				#( #ranges )*
				_ => #catch_all,
			}
		}
	} else {
		quote! { #catch_all }
	};

	//----[ Final result ]----------------------------------------------------//

	let tokens = quote! {
		|x: char| -> #return_type {
			#lookup
			#ranges
		}
	};

	tokens.into()
}
