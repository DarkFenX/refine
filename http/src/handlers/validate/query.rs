use crate::info::HValidInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct HValidInfoParams {
    pub(crate) validation: Option<HValidInfoMode>,
}
