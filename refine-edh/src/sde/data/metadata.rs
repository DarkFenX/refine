use serde::Deserialize;

#[derive(Deserialize)]
pub(in crate::sde) struct PMetadata {
    pub(in crate::sde) field_name: String,
    pub(in crate::sde) field_value: u64,
}
