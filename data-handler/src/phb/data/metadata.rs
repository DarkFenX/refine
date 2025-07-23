#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMetadata {
    pub(in crate::phb) field_name: String,
    pub(in crate::phb) field_value: u32,
}
