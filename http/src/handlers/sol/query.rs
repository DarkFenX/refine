use crate::info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSolInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct HSolInfoParams {
    pub(crate) sol: Option<HSolInfoMode>,
    pub(crate) fleet: Option<HFleetInfoMode>,
    pub(crate) fit: Option<HFitInfoMode>,
    pub(crate) item: Option<HItemInfoMode>,
}
