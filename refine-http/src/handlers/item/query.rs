use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct HItemInfoParams {
    pub(super) item: Option<rs::ItemInfoMode>,
}
