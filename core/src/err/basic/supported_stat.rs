use crate::{def::ItemId, svc::err::KeyedItemKindVsStatError, ud::UItems};

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} does not support requested stat")]
pub struct SupportedStatError {
    pub item_id: ItemId,
}
impl SupportedStatError {
    pub(crate) fn from_svc_err(u_items: &UItems, svc_err: KeyedItemKindVsStatError) -> Self {
        Self {
            item_id: u_items.id_by_key(svc_err.item_key),
        }
    }
}
