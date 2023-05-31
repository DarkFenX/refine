#[derive(Debug, serde::Deserialize)]
pub(crate) struct Metadata {
    pub(crate) field_name: String,
    pub(crate) field_value: u32,
}
