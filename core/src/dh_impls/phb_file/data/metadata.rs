#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Metadata {
    pub(in super::super) field_name: String,
    pub(in super::super) field_value: u32,
}
