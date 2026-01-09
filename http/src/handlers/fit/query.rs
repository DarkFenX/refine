use serde::Deserialize;

use crate::info::{HFitInfoMode, HItemInfoMode};

#[derive(Deserialize)]
pub(crate) struct HFitInfoParams {
    pub(in crate::handlers::fit) fit: Option<HFitInfoMode>,
    pub(in crate::handlers::fit) item: Option<HItemInfoMode>,
}
