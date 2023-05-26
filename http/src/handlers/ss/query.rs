use crate::info::{FitInfoMode, ItemInfoMode, SolSysInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct SolSysInfoParams {
    pub(crate) ss: Option<SolSysInfoMode>,
    pub(crate) fit: Option<FitInfoMode>,
    pub(crate) item: Option<ItemInfoMode>,
}
