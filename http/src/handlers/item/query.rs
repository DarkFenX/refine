use crate::info::ItemInfoMode;

#[derive(serde::Deserialize)]
pub(crate) struct ItemInfoParams {
    pub(crate) item: Option<ItemInfoMode>,
}
