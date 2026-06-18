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
    pub(crate) fn from_svc_err(svc_err: UItemLoadedError, u_items: &UItems) -> Self {
        Self {
            item_id: u_items.ext_id_by_int_id(svc_err.item_uid),
        }
    }
}
