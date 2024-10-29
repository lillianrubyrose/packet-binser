use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, DeriveInput, Error, Expr, ExprLit, Ident, Index, Lit, Result};

pub fn impl_packet(input: &DeriveInput) -> Result<TokenStream> {
	let ident = input.ident.clone();
	match &input.data {
		Data::Struct(data) => Ok(impl_struct(&input.attrs, data, &ident)?),
		_ => Err(Error::new_spanned(
			ident,
			"Packet is only implemented for structs currently",
		)),
	}
}

fn impl_struct(attrs: &[Attribute], data: &DataStruct, ident: &Ident) -> Result<TokenStream> {
	let mut header: Option<u64> = None;

	for attr in attrs {
		if attr.path().is_ident("header") {
			let nv = attr.meta.require_name_value()?;
			let Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) = &nv.value else {
				return Err(Error::new_spanned(&nv.value, "#[header] value must be an integer"));
			};

			header = Some(int.base10_parse()?);
		}
	}

	let Some(header) = header else {
		return Err(Error::new_spanned(ident, "#[header] must be present"));
	};

	let mut i = 0;
	let mut tuple_struct = false;
	let fields_serialize = data.fields.iter().map(|field| {
		let idx = Index::from(i);
		let tokens = if let Some(field_ident) = &field.ident {
			quote! { ::packet_binser::Binser::serialize(&self.#field_ident, buffer)?; }
		} else {
			tuple_struct = true;
			quote! { ::packet_binser::Binser::serialize(&self.#idx, buffer)?; }
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

	#[cfg(feature = "variable-width-lengths")]
	let serialize_header = quote! { ::packet_binser::varint::Variable::<u64>(#header).serialize(buffer)?; };
	// TODO: wtf to do here
	#[cfg(not(feature = "variable-width-lengths"))]
	let serialize_header = quote! { (#header as u16).serialize(buffer)?; };
	let serialize_fn = quote! {
		fn serialize<B: ::packet_binser::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), ::packet_binser::Error> {
		  #serialize_header
		  #( #fields_serialize )*
		  Ok(())
		}
	};

	let deserialize_fn = if tuple_struct {
		quote! {
		  fn deserialize<B: ::packet_binser::BytesReadExt>(buffer: &mut B) -> Result<Self, ::packet_binser::Error> {
			  Ok(Self(#( #fields_deserialize )*))
		  }
		}
	} else {
		quote! {
		  fn deserialize<B: ::packet_binser::BytesReadExt>(buffer: &mut B) -> Result<Self, ::packet_binser::Error> {
			  Ok(Self { #( #fields_deserialize )* })
		  }
		}
	};

	Ok(quote! {
	   impl ::packet_binser::Binser for #ident {
		   #serialize_fn
		   #deserialize_fn
	   }
	}
	.into())
}
