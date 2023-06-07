use crate::info::{HFitInfoMode, HItemInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct FitInfoParams {
    pub(crate) fit: Option<HFitInfoMode>,
    pub(crate) item: Option<HItemInfoMode>,
}
