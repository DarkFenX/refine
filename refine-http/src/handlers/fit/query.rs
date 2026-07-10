use serde::Deserialize;

use crate::info::{HFitInfoMode, HItemInfoMode};

#[derive(Deserialize)]
pub(crate) struct HFitInfoParams {
    pub(super) fit: Option<HFitInfoMode>,
    pub(super) item: Option<HItemInfoMode>,
}
