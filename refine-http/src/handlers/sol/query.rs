use serde::Deserialize;

use crate::info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSolInfoMode};

#[derive(Deserialize)]
pub(crate) struct HSolInfoParams {
    pub(super) sol: Option<HSolInfoMode>,
    pub(super) fleet: Option<HFleetInfoMode>,
    pub(super) fit: Option<HFitInfoMode>,
    pub(super) item: Option<HItemInfoMode>,
}
