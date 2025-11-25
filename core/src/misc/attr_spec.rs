use crate::{ad::AAttrId, ud::UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(crate) item_key: UItemKey,
    pub(crate) attr_id: AAttrId,
}
impl AttrSpec {
    pub(crate) fn new(item_key: UItemKey, attr_id: AAttrId) -> Self {
        Self { item_key, attr_id }
    }
}
