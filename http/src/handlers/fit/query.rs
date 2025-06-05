use crate::info::{HFitInfoMode, HItemInfoMode};

#[derive(serde::Deserialize)]
pub(crate) struct HFitInfoParams {
    pub(in crate::handlers::fit) fit: Option<HFitInfoMode>,
    pub(in crate::handlers::fit) item: Option<HItemInfoMode>,
}
