use crate::{def::ItemId, misc::AttrId};

pub struct AffectorInfo {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}
