use crate::info::HItemInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct HItemInfoParams {
    pub(crate) item: Option<HItemInfoMode>,
}
