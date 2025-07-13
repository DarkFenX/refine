use crate::{def::ItemId, svc::err::KeyedItemLoadedError, uad::UadItems};

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not loaded")]
pub struct ItemLoadedError {
    pub item_id: ItemId,
}
impl ItemLoadedError {
    pub(crate) fn from_svc_err(uad_items: &UadItems, svc_err: KeyedItemLoadedError) -> Self {
        Self {
            item_id: uad_items.id_by_key(svc_err.item_key),
        }
    }
}
