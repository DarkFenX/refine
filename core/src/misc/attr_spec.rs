use crate::{rd::RAttrKey, ud::UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AttrSpec {
    pub(crate) item_key: UItemId,
    pub(crate) attr_key: RAttrKey,
}
impl AttrSpec {
    pub(crate) fn new(item_key: UItemId, attr_key: RAttrKey) -> Self {
        Self { item_key, attr_key }
    }
}
