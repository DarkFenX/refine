use crate::{api::AttrId, ud::ItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Affector {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}
