use serde::Deserialize;

#[derive(Deserialize)]
pub(in crate::sde) struct SMetadata {
    #[serde(rename = "_key")]
    pub(in crate::sde) id: String,
    #[serde(rename = "buildNumber")]
    pub(in crate::sde) build_number: u64,
}
