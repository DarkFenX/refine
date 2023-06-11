use crate::info::{HFitInfoMode, HItemInfoMode, HSsInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct HSsInfoParams {
    pub(crate) ss: Option<HSsInfoMode>,
    pub(crate) fit: Option<HFitInfoMode>,
    pub(crate) item: Option<HItemInfoMode>,
}
