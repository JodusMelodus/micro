use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Path, Type, parse_macro_input};

#[proc_macro_derive(FromDTO, attributes(from))]
pub fn from_dto_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let from_path = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("from"))
        .and_then(|a| a.parse_args::<Path>().ok())
        .expect("The #[from(path)] attribute is required for FromDTO");

    let body = match input.data {
        Data::Struct(s) => {
            let fields = if let Fields::Named(f) = s.fields {
                f.named
            } else {
                panic!("FromDTO only supports structs with named fields");
            };

            let field_mappings = fields.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
        
                if is_type_name(field_type, "Vec") {
                    quote! {
                        #field_name: value.#field_name.into_iter().map(Into::into).collect::<#field_type>()
                    }
                } else if is_type_name(field_type, "Option") {
                    quote! {
                        #field_name: value.#field_name.map(Into::into)
                    }
                } else {
                    quote! {#field_name: value.#field_name.into()}
                }
            });

            quote! {
                Self {
                    #( #field_mappings, )*
                }
            }
        }
        Data::Enum(e) => {
            let arms = e.variants.iter().map(|variant| {
                let var_ident = &variant.ident;
                
                match &variant.fields {
                    Fields::Unit => {
                        quote! { #from_path::#var_ident => Self::#var_ident }
                    }
                    Fields::Unnamed(fields) => {
                        let idents: Vec<_> = (0..fields.unnamed.len())
                            .map(|i| quote::format_ident!("v{}", i))
                            .collect();
                        quote! {
                            #from_path::#var_ident(#(#idents),*) => Self::#var_ident(#(#idents.into()),*)
                        }
                    }
                    Fields::Named(fields) => {
                        let idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                        quote! {
                            #from_path::#var_ident { #(#idents),* } => Self::#var_ident { 
                                #(#idents: #idents.into()),* }
                        }
                    }
                }
            }).collect::<Vec<_>>();

            quote! {
                match value {
                    #( #arms, )*
                }
            }
        }
        _ => panic!("Unions are not supported by FromDTO")
    };


    let output = quote! {
        impl From<#from_path> for #name {
            fn from(value: #from_path) -> Self {
                #body
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
