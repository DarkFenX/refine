use crate::{
    defs::{EEffectId, EItemId},
    util::Named,
};

/// An EVE item type-effect relation.
pub struct EItemEffect {
    /// Refers an item type involved in the relation.
    pub item_id: EItemId,
    /// Refers an effect involved in the relation.
    pub effect_id: EEffectId,
    /// Defines if the effect is default to the item or not.
    pub is_default: bool,
}
impl EItemEffect {
    /// Make a new EVE item-effect relation out of passed data.
    pub fn new(item_id: EItemId, effect_id: EEffectId, is_default: bool) -> Self {
        Self {
            item_id,
            effect_id,
            is_default,
        }
    }
}
impl Named for EItemEffect {
    fn get_name() -> &'static str {
        "EItemEffect"
    }
}
