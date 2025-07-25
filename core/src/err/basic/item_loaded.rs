use crate::{def::ItemId, svc::err::KeyedItemLoadedError, ud::UItems};

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not loaded")]
pub struct ItemLoadedError {
    pub item_id: ItemId,
}
impl ItemLoadedError {
    pub(crate) fn from_svc_err(u_items: &UItems, svc_err: KeyedItemLoadedError) -> Self {
        Self {
            item_id: u_items.id_by_key(svc_err.item_key),
        }
    }
}
