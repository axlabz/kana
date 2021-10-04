use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn answer(_input: TokenStream) -> TokenStream {
	TokenStream::from(quote! {
		42
	})
}
