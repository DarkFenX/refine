use crate::def::{AttrId, ItemId};

pub struct AffectorInfo {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}
