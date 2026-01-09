use serde::Deserialize;

use crate::info::HItemInfoMode;

#[derive(Deserialize)]
pub(crate) struct HItemInfoParams {
    pub(in crate::handlers::item) item: Option<HItemInfoMode>,
}
