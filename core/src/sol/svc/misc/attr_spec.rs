use crate::{ad, sol::ItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(in crate::sol::svc) item_key: ItemKey,
    pub(in crate::sol::svc) a_attr_id: ad::AAttrId,
}
impl AttrSpec {
    pub(crate) fn new(item_key: ItemKey, a_attr_id: ad::AAttrId) -> Self {
        Self { item_key, a_attr_id }
    }
}
