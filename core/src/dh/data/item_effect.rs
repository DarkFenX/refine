use crate::{util::Named, ReeInt};

/// An item type - dogma effect relation.
#[derive(Debug)]
pub struct ItemEffect {
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers a dogma effect involved in the relation.
    pub effect_id: ReeInt,
    /// Defines if the effect is default to the item or not.
    pub is_default: bool,
}
impl ItemEffect {
    /// Make a new item-effect relation out of passed data.
    pub fn new(item_id: ReeInt, effect_id: ReeInt, is_default: bool) -> ItemEffect {
        ItemEffect {
            item_id,
            effect_id,
            is_default,
        }
    }
}
impl Named for ItemEffect {
    fn get_name() -> &'static str {
        "dh::ItemEffect"
    }
}
