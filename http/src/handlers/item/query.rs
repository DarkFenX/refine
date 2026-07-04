use serde::Deserialize;

use crate::info::HItemInfoMode;

#[derive(Deserialize)]
pub(crate) struct HItemInfoParams {
    pub(super) item: Option<HItemInfoMode>,
}
