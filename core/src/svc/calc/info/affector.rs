use crate::{api::AttrId, def::ItemId};

pub struct Affector {
    pub item_id: ItemId,
    pub attr_id: Option<AttrId>,
}
