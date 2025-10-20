use heck::ToSnakeCase;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Expr, Lit, Meta, parse_macro_input};

#[proc_macro_derive(Named, attributes(named))]
pub fn named(input: TokenStream) -> TokenStream {
    derive_named(input)
}

fn derive_named(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(_) | Data::Enum(_) => {}
        _ => {
            return syn::Error::new_spanned(input.ident, "no struct or enum")
                .to_compile_error()
                .into();
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let ident = &input.ident;
    let mut name = ident.to_string();

    for attr in input.attrs.iter() {
        if attr.path().is_ident("named") {
            match attr.parse_args() {
                Ok(Meta::NameValue(ref nv)) => {
                    if nv.path.is_ident("name") {
                        if let Expr::Lit(ref lit) = nv.value {
                            if let Lit::Str(ref s) = lit.lit {
                                name = s.value();
                            }
                        }
                    }
                }
                Ok(Meta::Path(ref p)) => {
                    if p.is_ident("snake_case") {
                        name = name.to_snake_case();
                    } else if p.is_ident("lowercase") {
                        name = name.to_lowercase();
                    }
                }
                _ => {}
            }
        }
    }

    quote::quote! {
        impl #impl_generics ::tynamed::Named for #ident #ty_generics #where_clause {
            fn name() -> &'static str {
                #name
            }
        }
    }
    .into()
}
