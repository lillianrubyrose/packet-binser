mod binser;
mod packet;

use binser::impl_binser;
use packet::impl_packet;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Binser)]
pub fn binser(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	match impl_binser(input) {
		Ok(tokens) => tokens,
		Err(err) => err.into_compile_error().into(),
	}
}

#[proc_macro_derive(Packet, attributes(header))]
pub fn packet(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	match impl_packet(input) {
		Ok(tokens) => tokens,
		Err(err) => err.into_compile_error().into(),
	}
}
