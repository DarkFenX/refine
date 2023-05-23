use crate::info::{FitInfoMode, ItemInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct FitInfoParams {
    pub(crate) fit: Option<FitInfoMode>,
    pub(crate) item: Option<ItemInfoMode>,
}
