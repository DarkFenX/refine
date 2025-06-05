use crate::info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSolInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct HSolInfoParams {
    pub(in crate::handlers::sol) sol: Option<HSolInfoMode>,
    pub(in crate::handlers::sol) fleet: Option<HFleetInfoMode>,
    pub(in crate::handlers::sol) fit: Option<HFitInfoMode>,
    pub(in crate::handlers::sol) item: Option<HItemInfoMode>,
}
