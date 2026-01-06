use crate::{api::AttrId, ud::ItemId};

pub struct Affector {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}
