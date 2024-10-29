#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]

mod binser;

use binser::impl_binser;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Binser)]
pub fn binser(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	match impl_binser(&input) {
		Ok(tokens) => tokens,
		Err(err) => err.into_compile_error().into(),
	}
}
