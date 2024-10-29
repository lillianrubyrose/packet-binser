use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Expr, ExprLit, Ident, Index, Lit, Result};

pub fn impl_binser(input: DeriveInput) -> Result<TokenStream> {
	let ident = input.ident.clone();
	match &input.data {
		Data::Struct(data) => Ok(impl_struct(data, ident)?),
		Data::Enum(data) => Ok(impl_enum(data, ident)?),
		_ => Err(Error::new_spanned(
			ident,
			"Packet is only implemented for structs currently",
		)),
	}
}

fn impl_enum(data: &DataEnum, ident: Ident) -> Result<TokenStream> {
	let mut variant_idx = 0;
	let variants_serialize = data
		.variants
		.iter()
		.map(|variant| {
			let ident = &variant.ident;
			if let Some((_eq, val)) = &variant.discriminant {
				match val {
					Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
						let value = int.base10_parse::<u16>()?;
						variant_idx = value;
						Ok(quote! { Self::#ident => #value, })
					}
					_ => Err(Error::new_spanned(val, "Expected integer literal")),
				}
			} else {
				let q = quote! { Self::#ident => #variant_idx, };
				variant_idx += 1;
				Ok(q)
			}
		})
		.collect::<Result<Vec<_>>>()?;

	variant_idx = 0;
	let variants_deserialize = data
		.variants
		.iter()
		.map(|variant| {
			let ident = &variant.ident;
			if let Some((_eq, val)) = &variant.discriminant {
				match val {
					Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
						let value = int.base10_parse::<u16>()?;
						variant_idx = value + 1;
						Ok(quote! { #value => Self::#ident, })
					}
					_ => Err(Error::new_spanned(val, "Expected integer literal")),
				}
			} else {
				let q = quote! { #variant_idx => Self::#ident, };
				variant_idx += 1;
				Ok(q)
			}
		})
		.collect::<Result<Vec<_>>>()?;

	let serialize_fn = quote! {
		fn serialize<B: ::packet_binser::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), ::packet_binser::Error> {
		  buffer.write_u16(match self {
			#( #variants_serialize )*
		  } as u16)?;
		  Ok(())
		}
	};

	let deserialize_fn = quote! {
		fn deserialize<B: ::packet_binser::BytesReadExt>(buffer: &mut B) -> Result<Self, ::packet_binser::Error> {
		  Ok(match buffer.read_u16()? {
			#( #variants_deserialize )*
			other => Err(::packet_binser::Error::InvalidPacketData(format!("Invalid enum variant: {other}")))?,
		  })
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

fn impl_struct(data: &DataStruct, ident: Ident) -> Result<TokenStream> {
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

	let serialize_fn = quote! {
		fn serialize<B: ::packet_binser::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), ::packet_binser::Error> {
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
