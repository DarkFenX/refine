use crate::{ad, sol::ItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) struct AttrSpec {
    pub(in crate::sol::svc::calc) item_id: ItemId,
    pub(in crate::sol::svc::calc) a_attr_id: ad::AAttrId,
}
