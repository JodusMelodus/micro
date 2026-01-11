use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Path, Type, parse_macro_input};

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

    let field_mappings = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        if is_type_name(field_type, "Vec") {
            quote! {
                #field_name: value.#field_name.into_iter().map(Into::into).collect()
            }
        } else if is_type_name(field_type, "Option") {
            quote! {
                #field_name: value.#field_name.into_iter().map(Into::into).collect()
            }
        } else {
            quote! {#field_name: value.#field_name.into()}
        }
    });

    let output = quote! {
        impl From<#path> for #name {
            fn from(value: #path) -> Self {
                Self {
                    #( #field_mappings,)*
                }
            }
        }
    };

    TokenStream::from(output)
}

fn is_type_name(ty: &Type, name: &str) -> bool {
    if let Type::Path(tp) = ty {
        tp.path.segments.iter().any(|s| s.ident == name)
    } else {
        false
    }
}
