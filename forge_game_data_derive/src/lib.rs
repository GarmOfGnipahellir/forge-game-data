extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Lit, Meta, NestedMeta};

#[proc_macro_derive(Entity, attributes(entity))]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let mut name = None;
    let mut display_name = None;
    for attr in ast.attrs.iter().filter_map(|attr| attr.parse_meta().ok()) {
        let list = if let Meta::List(list) = attr {
            list
        } else {
            continue;
        };

        if list
            .path
            .get_ident()
            .map(|ident| ident != "entity")
            .unwrap_or(true)
        {
            continue;
        }

        for nested_attr in list.nested.iter() {
            match nested_attr {
                NestedMeta::Meta(Meta::NameValue(name_value)) => {
                    let name_value =
                        if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested_attr {
                            name_value
                        } else {
                            continue;
                        };

                    let ident = if let Some(ident) = name_value.path.get_ident() {
                        ident
                    } else {
                        continue;
                    };

                    match ident.to_string().as_str() {
                        "name" => {
                            if let Lit::Str(value) = &name_value.lit {
                                name = Some(value.value());
                            }
                        }
                        "display_name" => {
                            if let Lit::Str(value) = &name_value.lit {
                                display_name = Some(value.value());
                            }
                        }
                        _ => continue,
                    }
                }
                NestedMeta::Meta(Meta::List(list)) => {
                    if list
                        .path
                        .get_ident()
                        .map(|ident| ident != "properties")
                        .unwrap_or(true)
                    {
                        continue;
                    }

                    for nested_attr in list.nested.iter() {
                        let list = if let NestedMeta::Meta(Meta::List(list)) = nested_attr {
                            list
                        } else {
                            continue;
                        };

                        let ident = if let Some(ident) = list.path.get_ident() {
                            ident
                        } else {
                            continue;
                        };

                        println!("{}", ident);
                    }
                }
                _ => continue,
            }
        }
    }

    let ident = &ast.ident;

    let name = name.unwrap_or(ident.to_string());
    let display_name = display_name.unwrap_or_default();

    let gen = quote! {
        impl Entity for #ident {
            fn name() -> &'static str {
                #name
            }

            fn display_name() -> &'static str {
                #display_name
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
