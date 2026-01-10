use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = quote::format_ident!("{}Builder", name);
    let fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("Only structs supported"),
    };
    let field_idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let expanded = quote! {
        pub struct #builder_name {
            #(#field_idents: Option<#field_types>),*
        }
        impl #builder_name {
            pub fn new() -> Self {
                Self {
                    #(#field_idents: None),*
                }
            }
            #(
                pub fn #field_idents(mut self, v: #field_types) -> Self {
                    self.#field_idents = Some(v);
                    self
                }
            )*
            pub fn build(self) -> Result<#name, &'static str> {
                Ok(#name {
                    #(#field_idents: self.#field_idents.ok_or(concat!("missing ", stringify!(#field_idents)))?),*
                })
            }
        }
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }
    };
    TokenStream::from(expanded)
}