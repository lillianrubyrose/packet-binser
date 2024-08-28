mod packet;
mod packet_serde;

use packet::impl_packet;
use packet_serde::impl_packet_serde;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PacketSerde)]
pub fn packet_serde(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	match impl_packet_serde(input) {
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
