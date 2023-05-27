use crate::info::{FitInfoMode, ItemInfoMode, SsInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct SsInfoParams {
    pub(crate) ss: Option<SsInfoMode>,
    pub(crate) fit: Option<FitInfoMode>,
    pub(crate) item: Option<ItemInfoMode>,
}
