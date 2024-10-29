use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Expr, ExprLit, Ident, Index, Lit, Result};

pub fn impl_binser(input: &DeriveInput) -> Result<TokenStream> {
	let ident = input.ident.clone();
	match &input.data {
		Data::Struct(data) => Ok(impl_struct(data, &ident)),
		Data::Enum(data) => Ok(impl_enum(data, &ident)?),
		Data::Union(_) => Err(Error::new_spanned(
			ident,
			"Packet is only implemented for structs currently",
		)),
	}
}

fn impl_enum(data: &DataEnum, ident: &Ident) -> Result<TokenStream> {
	let mut variant_idx = 0;
	let variants_serialize = data
		.variants
		.iter()
		.map(|variant| {
			let ident = &variant.ident;

			let idx = if let Some((_eq, val)) = &variant.discriminant {
				match val {
					Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
						let value = int.base10_parse::<u16>()?;
						variant_idx = value;
						value
					}
					_ => return Err(Error::new_spanned(val, "Expected integer literal")),
				}
			} else {
				variant_idx += 1;
				variant_idx
			};

			if variant.fields.is_empty() {
				Ok(quote! { Self::#ident => {
					buffer.write_u16(#idx)?;
				}})
			} else {
				let mut field_idx = 'a';
				let fields = variant
					.fields
					.iter()
					.map(|field| {
						if let Some(field_ident) = &field.ident {
							quote! { #field_ident.serialize(buffer)?; }
						} else {
							let ident = format_ident!("{}", field_idx);
							field_idx = (field_idx as u8 + 1) as char;
							quote! { #ident.serialize(buffer)?; }
						}
					})
					.collect::<Vec<_>>();
				let variant_is_struct = variant.fields.iter().next().is_some_and(|field| field.ident.is_some());
				let variant_match = if variant_is_struct {
					let field_idents = variant.fields.iter().map(|field| {
						let ident = field.ident.as_ref().unwrap();
						quote! { #ident, }
					});
					quote! { Self::#ident { #(#field_idents)* } }
				} else {
					let mut idx = 'a';
					let field_idents = variant.fields.iter().map(|_| {
						let ident = format_ident!("{}", idx);
						let q = quote! { #ident, };
						idx = (idx as u8 + 1) as char;
						q
					});
					quote! { Self::#ident( #(#field_idents)* ) }
				};
				Ok(quote! { #variant_match => { buffer.write_u16(#idx)?; #( #fields )*; } })
			}
		})
		.collect::<Result<Vec<_>>>()?;

	variant_idx = 0;
	let variants_deserialize = data
		.variants
		.iter()
		.map(|variant| {
			let ident = &variant.ident;
			let idx = if let Some((_eq, val)) = &variant.discriminant {
				match val {
					Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
						let value = int.base10_parse::<u16>()?;
						variant_idx = value;
						value
					}
					_ => return Err(Error::new_spanned(val, "Expected integer literal")),
				}
			} else {
				variant_idx += 1;
				variant_idx
			};

			if variant.fields.is_empty() {
				Ok(quote! { #idx => Self::#ident, })
			} else {
				let fields = variant
					.fields
					.iter()
					.map(|field| {
						if let Some(field_ident) = &field.ident {
							quote! { #field_ident: ::packet_binser::Binser::deserialize(buffer)?, }
						} else {
							quote! { ::packet_binser::Binser::deserialize(buffer)?, }
						}
					})
					.collect::<Vec<_>>();
				let variant_is_struct = variant.fields.iter().next().is_some_and(|field| field.ident.is_some());

				if variant_is_struct {
					Ok(quote! { #idx => Self::#ident { #( #fields )* }, })
				} else {
					Ok(quote! { #idx => Self::#ident(#( #fields )*), })
				}
			}
		})
		.collect::<Result<Vec<_>>>()?;

	let serialize_fn = quote! {
		fn serialize<B: ::packet_binser::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), ::packet_binser::Error> {
		  match self {
			#( #variants_serialize )*
		  }
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

fn impl_struct(data: &DataStruct, ident: &Ident) -> TokenStream {
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

	quote! {
	   impl ::packet_binser::Binser for #ident {
		   #serialize_fn
		   #deserialize_fn
	   }
	}
	.into()
}
