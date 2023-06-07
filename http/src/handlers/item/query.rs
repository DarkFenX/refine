use crate::info::HItemInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct ItemInfoParams {
    pub(crate) item: Option<HItemInfoMode>,
}
