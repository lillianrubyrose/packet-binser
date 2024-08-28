use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Ident, Index, Result};

pub fn impl_packet_serde(input: DeriveInput) -> Result<TokenStream> {
	let ident = input.ident.clone();
	match &input.data {
		Data::Struct(data) => Ok(impl_struct(data, ident)?),
		_ => Err(Error::new_spanned(
			ident,
			"Packet is only implemented for structs currently",
		)),
	}
}

fn impl_struct(data: &DataStruct, ident: Ident) -> Result<TokenStream> {
	let mut i = 0;
	let mut tuple_struct = false;
	let fields_serialize = data.fields.iter().map(|field| {
		let idx = Index::from(i);
		let tokens = if let Some(field_ident) = &field.ident {
			quote! { ::packet_binser::PacketSerde::serialize(&self.#field_ident, buffer)?; }
		} else {
			tuple_struct = true;
			quote! { ::packet_binser::PacketSerde::serialize(&self.#idx, buffer)?; }
		};

		i += 1;
		tokens
	});

	let fields_deserialize = data.fields.iter().map(|field| {
		let ty = &field.ty;
		if let Some(field_ident) = &field.ident {
			quote! { #field_ident: #ty::deserialize(buffer)?, }
		} else {
			quote! { #ty::deserialize(buffer)?, }
		}
	});

	let serialize_fn = quote! {
		fn serialize<B: ::packet_binser::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), ::packet_binser::lbytes::Error> {
		  #( #fields_serialize )*
		  Ok(())
		}
	};

	let deserialize_fn = if tuple_struct {
		quote! {
		  fn deserialize<B: ::packet_binser::BytesReadExt>(buffer: &mut B) -> Result<Self, ::packet_binser::lbytes::Error> {
			  Ok(Self(#( #fields_deserialize )*))
		  }
		}
	} else {
		quote! {
		  fn deserialize<B: ::packet_binser::BytesReadExt>(buffer: &mut B) -> Result<Self, ::packet_binser::lbytes::Error> {
			  Ok(Self { #( #fields_deserialize )* })
		  }
		}
	};

	Ok(quote! {
	   impl ::packet_binser::PacketSerde for #ident {
		   #serialize_fn
		   #deserialize_fn
	   }
	}
	.into())
}
