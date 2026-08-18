use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ValParams {
    #[serde(default)]
    pub(super) validation: rs::val::ValInfoMode,
}
