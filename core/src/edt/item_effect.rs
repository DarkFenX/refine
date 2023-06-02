use crate::{defs::ReeInt, util::Named};

/// An EVE item type-effect relation.
#[derive(Debug)]
pub struct EItemEffect {
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers an effect involved in the relation.
    pub effect_id: ReeInt,
    /// Defines if the effect is default to the item or not.
    pub is_default: bool,
}
impl EItemEffect {
    /// Make a new EVE item-effect relation out of passed data.
    pub fn new(item_id: ReeInt, effect_id: ReeInt, is_default: bool) -> Self {
        Self {
            item_id,
            effect_id,
            is_default,
        }
    }
}
impl Named for EItemEffect {
    fn get_name() -> &'static str {
        "edt::EItemEffect"
    }
}
