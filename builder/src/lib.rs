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
        panic!("Only Struct's with named fields allowed.");
    };

    let optionized = fields.iter().map(|x| {

        let mut segments = Punctuated::<PathSegment, syn::token::Colon2>::new();

        let old_path = if let Type::Path(TypePath{ path, ..}) = x.ty {
            path
        } else {
            panic!("No type path found.")
        };

        segments.push(PathSegment{
            ident: Ident::new(&format!("Option<{}>", old_path), x.span()),
            arguments: PathArguments::None
        });

        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments
            }
        });

        let field = Field {
            attrs: vec![],
            ident: x.ident.clone(),
            vis: Visibility::Inherited,
            colon_token: x.colon_token,
            ty,
        };

        field
    });

    eprintln!("{:#?}", fields);

    let code = quote! {

        pub struct #builder_ident {
            #(#optionized),*
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
