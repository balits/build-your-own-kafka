extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(WireLen)]
pub fn derive_wire_len(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let sum = helpers::sum_wire_len(&ast.data);

    let gen = quote! {
        impl crate::WireLen for #name {
            fn wire_len(&self) -> usize {
                #sum
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(Encoder)]
pub fn derive_encoder(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let inner_impl = helpers::impl_encode(&ast.data);

    let gen = quote! {
        impl crate::Encoder for #name {
            fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
                use bytes::BufMut;
                #inner_impl
                Ok(())
            }
        }
    };

    TokenStream::from(gen)
}

mod helpers {
    use proc_macro2::TokenStream as TokenStream2;
    use quote::{format_ident, quote, quote_spanned};
    use syn::{spanned::Spanned, Data, Fields, Index, Type, Ident};

    pub fn impl_encode(data: &Data) -> TokenStream2 {
        match *data {
            Data::Struct(ref st) => match st.fields {
                Fields::Named(ref fields) => {
                    let encodes = fields.named.iter().map(|f| {
                        let fname = f.ident.as_ref().unwrap();
                        let ftype = &f.ty;

                        if let Some(method) = primitive_encode_method(ftype) {
                            quote_spanned! {f.span() =>
                                dest.#method(self.#fname);
                            }
                        } else {
                            quote! { self.#fname.encode(dest)?; }
                        }
                    });

                    quote! {
                        #(#encodes)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let encodes = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        let ftype = &f.ty;

                        if let Some(method) = primitive_encode_method(ftype) {
                            quote! {
                                dest.#method(self.#index);
                            }
                        } else {
                            quote! { self.#index.encode(dest)?; }
                        }
                    });

                    quote! {
                        #(#encodes)*
                    }
                }
                Fields::Unit => {
                    quote! {}
                }
            },
            _ => quote! { compile_error!("Only structs are supported") },
        }
    }

    fn primitive_encode_method(t: &Type) -> Option<Ident> {
        if let syn::Type::Path(path) = t {
            let ident = &path.path.segments.last().unwrap().ident;
            match ident.to_string().as_str() {
                "u8" => Some(format_ident! { "put_u8" }),
                "u16" => Some(format_ident! { "put_u16" }),
                "u32" => Some(format_ident! { "put_u32" }),
                "u64" => Some(format_ident! { "put_u64" }),
                "i8" => Some(format_ident! { "put_i8" }),
                "i16" => Some(format_ident! { "put_i16" }),
                "i32" => Some(format_ident! { "put_i32" }),
                "i64" => Some(format_ident! { "put_i64" }),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn sum_wire_len(data: &Data) -> TokenStream2 {
        match *data {
            Data::Enum(_) | Data::Union(_) => unimplemented!(),
            Data::Struct(ref st) => match st.fields {
                Fields::Unit => quote!(0),
                Fields::Unnamed(ref fields) => {
                    let rec = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span() =>
                            crate::WireLen::wire_len(&self.#index)
                        }
                    });

                    quote! {
                        0 #(+ #rec)*
                    }
                }
                Fields::Named(ref fields) => {
                    let rec = fields.named.iter().map(|field| {
                        let fname = &field.ident;
                        quote_spanned! {field.span() =>
                            crate::WireLen::wire_len(&self.#fname)
                        }
                    });

                    quote! {
                        0 #(+ #rec)*
                    }
                }
            },
        }
    }
}
