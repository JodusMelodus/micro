use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Path, parse_macro_input};

#[proc_macro_derive(FromDTO, attributes(from))]
pub fn from_dto_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut from_path: Option<Path> = None;

    for attribute in &input.attrs {
        if attribute.path().is_ident("from") {
            if let Ok(path) = attribute.parse_args::<Path>() {
                from_path = Some(path)
            }
        }
    }

    let path = from_path.expect("The #[from(path)] attribute is required for FromDTO");

    let fields = if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            f.named
        } else {
            panic!("FromDTO only supports structs with named fields");
        }
    } else {
        panic!("FromDTO can only be used on structs");
    };

    let field_idents = fields.iter().map(|f| &f.ident);

    let output = quote! {
        impl From<#path> for #name {
            fn from(value: #path) -> Self {
                Self {
                    #(
                        #field_idents: value.#field_idents.into(),
                    )*
                }
            }
        }
    };

    TokenStream::from(output)
}
