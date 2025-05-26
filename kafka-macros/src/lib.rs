extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, Fields, Index};

#[proc_macro_derive(WireLen)]
pub fn derive_wire_len(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let sum = sum_wire_len(&ast.data);

    let gen = quote! {
        impl crate::WireLen for #name {
            fn wire_len(&self) -> usize {
                #sum
            }
        }
    };

    TokenStream::from(gen)
}

fn sum_wire_len(data: &Data) -> TokenStream2 {
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
                    let name = &field.ident;
                    quote_spanned! {field.span() =>
                        crate::WireLen::wire_len(&self.#name)
                    }
                });

                quote! {
                    0 #(+ #rec)*
                }
            }
        },
    }
}
