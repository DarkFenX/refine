use crate::{rd::RAttrId, ud::UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(crate) item_uid: UItemId,
    pub(crate) attr_rid: RAttrId,
}
impl AttrSpec {
    pub(crate) fn new(item_uid: UItemId, attr_rid: RAttrId) -> Self {
        Self { item_uid, attr_rid }
    }
}
