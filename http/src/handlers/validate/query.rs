use crate::info::HValidInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct HValidInfoParams {
    pub(in crate::handlers::validate) validation: Option<HValidInfoMode>,
}
