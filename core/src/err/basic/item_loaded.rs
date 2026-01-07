use crate::{
    svc::err::UItemLoadedError,
    ud::{ItemId, UItems},
};

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not loaded")]
pub struct ItemLoadedError {
    pub item_id: ItemId,
}
impl ItemLoadedError {
    pub(crate) fn from_svc_err(u_items: &UItems, svc_err: UItemLoadedError) -> Self {
        Self {
            item_id: u_items.xid_by_iid(svc_err.item_uid),
        }
    }
}
