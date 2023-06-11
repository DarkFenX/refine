use crate::info::{HFitInfoMode, HItemInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct HFitInfoParams {
    pub(crate) fit: Option<HFitInfoMode>,
    pub(crate) item: Option<HItemInfoMode>,
}
