use crate::{
    svc::err::UItemKindVsStatError,
    ud::{ItemId, UItems},
};

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} does not support requested stat")]
pub struct SupportedStatError {
    pub item_id: ItemId,
}
impl SupportedStatError {
    pub(crate) fn from_svc_err(svc_err: UItemKindVsStatError, u_items: &UItems) -> Self {
        Self {
            item_id: u_items.xid_by_iid(svc_err.item_uid),
        }
    }
}
