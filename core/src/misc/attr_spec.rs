use crate::{rd::RAttrId, ud::UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(crate) item_key: UItemId,
    pub(crate) attr_key: RAttrId,
}
impl AttrSpec {
    pub(crate) fn new(item_key: UItemId, attr_key: RAttrId) -> Self {
        Self { item_key, attr_key }
    }
}
