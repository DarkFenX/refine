use crate::info::HItemInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct HItemInfoParams {
    pub(in crate::handlers::item) item: Option<HItemInfoMode>,
}
