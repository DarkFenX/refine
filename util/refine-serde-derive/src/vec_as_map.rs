use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Type, parse_macro_input, spanned::Spanned, visit_mut::VisitMut};

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
    let ((key_name, key_type), (value_name, value_type), value_as) = get_key_value_fields(input)?;
    let mut value_as = value_as.unwrap_or_else(|| syn::parse_quote!(_));
    InferAsSame.visit_type_mut(&mut value_as);
    let item_type = &input.ident;
    Ok(quote! {
        impl ::refine_serde::AsMapEntry for #item_type {
            type Key = #key_type;
            type Value = #value_type;
            type ValueAs = #value_as;

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

fn get_key_value_fields(input: &DeriveInput) -> syn::Result<(FieldInfo, FieldInfo, Option<Type>)> {
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
    let mut value_as = None;
    for field in &fields.named {
        // Gather info about field
        let mut is_key = false;
        let mut is_value = false;
        let mut field_as = None;
        for attr in field.attrs.iter().filter(|attr| attr.path().is_ident("vec_map")) {
            attr.parse_nested_meta(|meta| {
                match () {
                    _ if meta.path.is_ident("key") => is_key = true,
                    _ if meta.path.is_ident("value") => is_value = true,
                    _ if meta.path.is_ident("serialize_as") => {
                        field_as = Some(meta.value()?.parse::<LitStr>()?.parse()?)
                    }
                    _ => return Err(meta.error("expected `key`, `value` or `serialize_as`")),
                }
                Ok(())
            })?;
        }
        // Compatibility checks
        if is_key && is_value {
            return Err(syn::Error::new(field.span(), "field cannot be both key and value"));
        }
        if field_as.is_some() && !is_value {
            return Err(syn::Error::new(
                field.span(),
                "`serialize_as` is supported only on value field",
            ));
        }
        // Duplication checks
        if is_key {
            if key.is_some() {
                return Err(syn::Error::new(field.span(), "key field has already been defined"));
            }
            key = field.ident.clone().map(|name| (name, field.ty.clone()));
        }
        if is_value {
            if value.is_some() {
                return Err(syn::Error::new(field.span(), "value field has already been defined"));
            }
            value = field.ident.clone().map(|name| (name, field.ty.clone()));
            value_as = field_as;
        }
    }
    // Completeness checks
    let key = key.ok_or_else(|| syn::Error::new(input.span(), "no field marked with #[vec_map(key)]"))?;
    let value = value.ok_or_else(|| syn::Error::new(input.span(), "no field marked with #[vec_map(value)]"))?;
    Ok((key, value, value_as))
}

// Convert placeholder to "::serde_with::Same"
struct InferAsSame;
impl VisitMut for InferAsSame {
    fn visit_type_mut(&mut self, node: &mut Type) {
        match node {
            Type::Infer(_) => *node = syn::parse_quote!(::serde_with::Same),
            _ => syn::visit_mut::visit_type_mut(self, node),
        }
    }
}
