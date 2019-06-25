extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct, FieldsNamed, Field, Visibility, Type, TypePath, Path, PathSegment, Ident, PathArguments};


use quote::quote;
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {

    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let builder_name = &format!("{}Builder", name);
    let builder_ident = syn::Ident::new(builder_name, name.span());

    let fields = if let Data::Struct(DataStruct{
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = ast.data {
        named
    } else {
        panic!("Only Structs with named fields allowed.");
    };

    let optionized_fields = fields.iter().map(|x| {

        let ident = &x.ident;
        let ty = &x.ty;

        quote! {
            #ident: ::std::option::Option<#ty>
        }
    });

    let builder_methods = fields.iter().map(|x| {

        let ident = &x.ident;
        let ty = &x.ty;

        quote! {
            pub fn #ident (&mut self, val: #ty) -> &mut Self {

                self.#ident = Some(val);
                self
            }
        }
    });

    let field_mapping = fields.iter().map(|x| {

        let ident = &x.ident;

        quote! {
            #ident: self.#ident.clone().ok_or("Option was None.")?
        }
    });

    let code = quote! {

        pub struct #builder_ident {

            #(#optionized_fields),*
        }

        impl #builder_ident {

            #(#builder_methods)*

            pub fn build(&self) -> ::std::result::Result<#name, Box<dyn ::std::error::Error>> {

                Ok(#name {
                    #(#field_mapping),*
                })
            }
        }

        impl #name {

            fn builder() -> #builder_ident {

                #builder_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    code.into()
}
