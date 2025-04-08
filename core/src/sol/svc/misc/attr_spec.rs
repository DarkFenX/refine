use crate::{ad, sol::ItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc) struct AttrSpec {
    pub(in crate::sol::svc) item_key: ItemKey,
    pub(in crate::sol::svc) a_attr_id: ad::AAttrId,
}
