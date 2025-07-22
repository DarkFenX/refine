use crate::{ad, uad::UadItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(crate) item_key: UadItemKey,
    pub(crate) a_attr_id: ad::AAttrId,
}
impl AttrSpec {
    pub(crate) fn new(item_key: UadItemKey, a_attr_id: ad::AAttrId) -> Self {
        Self { item_key, a_attr_id }
    }
}
