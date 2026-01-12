use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, PathArguments, Type, parse_macro_input};

#[proc_macro_derive(FromDTO, attributes(from))]
pub fn from_dto_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let from_types = input.attrs.iter().filter(|a|a.path().is_ident("from"))
        .filter_map(|a|a.parse_args::<Type>().ok())
        .collect::<Vec<_>>();

    if from_types.is_empty() {
        panic!("The #[from(Type)] attribute is required for FromDTO");
    }
    
    let expanded = from_types.iter().map(|from_type|{
        let from_type_stripped = strip_generics(from_type);

        let body = match &input.data {
            Data::Struct(s) => {
                let fields = if let Fields::Named(f) = &s.fields {
                    &f.named
                } else {
                    panic!("FromDTO only supports structs with named fields");
                };

                let field_mappings = fields.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;

                    if let Some(inner_vec_type) = get_inner_type_from_option_vec(field_type) {
                        quote!{
                            #field_name: value.#field_name.map(|v| v.into_iter().map(Into::into).collect::<#inner_vec_type>())
                        }
                    }else if is_type_name(field_type, "Vec") {
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
                            quote! { #from_type_stripped::#var_ident => Self::#var_ident }
                        }
                        Fields::Unnamed(fields) => {
                            let idents: Vec<_> = (0..fields.unnamed.len())
                                .map(|i| quote::format_ident!("v{}", i))
                                .collect();
                            quote! {
                                #from_type_stripped::#var_ident(#(#idents),*) => Self::#var_ident(#(#idents.into()),*)
                            }
                        }
                        Fields::Named(fields) => {
                            let idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                            quote! {
                                #from_type_stripped::#var_ident { #(#idents),* } => Self::#var_ident { 
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


        quote! {
            impl #impl_generics From<#from_type> for #name #ty_generics #where_clause {
                fn from(value: #from_type) -> Self {
                    #body
                }
            }
        }
    });

    TokenStream::from(quote! { #(#expanded)* })
}

fn get_inner_type_from_option_vec(ty: &syn::Type) -> Option<syn::Type> {
    let tp = if let syn::Type::Path(tp) = ty { tp } else { return None };
    let seg = tp.path.segments.last()?;
    
    if seg.ident != "Option" { return None; }

    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
        if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_tp))) = args.args.first() {
            let inner_seg = inner_tp.path.segments.last()?;
            if inner_seg.ident == "Vec" {
                return Some(syn::Type::Path(inner_tp.clone()));
            }
        }
    }
    None
}

fn strip_generics(ty: &Type) -> proc_macro2::TokenStream {
    if let Type::Path(tp) = ty {
        let mut path = tp.path.clone();
        for segment in &mut path.segments {
            segment.arguments = PathArguments::None
        }
        quote!(#path)
    } else {
        quote!(#ty)
    }
}

fn is_type_name(ty: &Type, name: &str) -> bool {
    if let Type::Path(tp) = ty {
        tp.path.segments.iter().any(|s| s.ident == name)
    } else {
        false
    }
}
