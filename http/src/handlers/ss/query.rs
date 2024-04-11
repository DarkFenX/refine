use crate::info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSsInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct HSsInfoParams {
    pub(crate) ss: Option<HSsInfoMode>,
    pub(crate) fleet: Option<HFleetInfoMode>,
    pub(crate) fit: Option<HFitInfoMode>,
    pub(crate) item: Option<HItemInfoMode>,
}
