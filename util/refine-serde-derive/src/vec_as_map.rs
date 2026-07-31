use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Type, parse_macro_input, spanned::Spanned};

pub fn vec_as_map_entry_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "VecAsMapEntry does not support generic types",
        ));
    }
    let ((key_name, key_type), (value_name, value_type)) = get_key_value_fields(input)?;
    let item_type = &input.ident;
    Ok(quote! {
        impl ::refine_serde::AsMapEntry for #item_type {
            type Key = #key_type;
            type Value = #value_type;

            fn get_key(&self) -> &Self::Key {
                &self.#key_name
            }
            fn get_value(&self) -> &Self::Value {
                &self.#value_name
            }
        }
    })
}

type FieldInfo = (Ident, Type);

fn get_key_value_fields(input: &DeriveInput) -> syn::Result<(FieldInfo, FieldInfo)> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(input.span(), "VecAsMapEntry supports only structs"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new(
            data.fields.span(),
            "VecAsMapEntry supports only structs with named fields",
        ));
    };
    let mut key = None;
    let mut value = None;
    for field in &fields.named {
        for attr in field.attrs.iter().filter(|attr| attr.path().is_ident("vec_map")) {
            attr.parse_nested_meta(|meta| {
                let target = match () {
                    _ if meta.path.is_ident("key") => &mut key,
                    _ if meta.path.is_ident("value") => &mut value,
                    _ => return Err(meta.error("expected `key` or `value`")),
                };
                match target {
                    Some(_) => Err(meta.error("field has already been defined")),
                    None => {
                        *target = field.ident.clone().map(|name| (name, field.ty.clone()));
                        Ok(())
                    }
                }
            })?;
        }
    }
    let key = key.ok_or_else(|| syn::Error::new(input.span(), "no field marked with #[vec_map(key)]"))?;
    let value = value.ok_or_else(|| syn::Error::new(input.span(), "no field marked with #[vec_map(value)]"))?;
    Ok((key, value))
}
