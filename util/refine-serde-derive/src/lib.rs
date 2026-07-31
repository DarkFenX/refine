use vec_as_map::vec_as_map_entry_impl;

mod vec_as_map;

/// Helper macro for `VecAsMap`, which designates which fields to use as key and as value. Value
/// can have optional serialization handler, similar to how `serde_with` specifies it.
///
/// ```ignore
/// #[derive(VecAsMapEntry)]
/// pub struct VecMember {
///     #[vec_map(key)]
///     pub field1: i32,
///     #[vec_map(value, serialize_as = "<type>")]
///     pub field2: i32,
/// }
/// ```
#[proc_macro_derive(VecAsMapEntry, attributes(vec_map))]
pub fn vec_as_map_entry(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    vec_as_map_entry_impl(input)
}
