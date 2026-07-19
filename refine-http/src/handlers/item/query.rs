use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ItemInfoParams {
    pub(super) item: Option<rs::ItemInfoMode>,
}
