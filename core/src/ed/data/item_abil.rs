use crate::{
    defs::{AbilId, Amount, AttrVal, ItemId},
    util::Named,
};

/// An EVE item type-fighter ability relation.
#[derive(Debug)]
pub struct EItemFighterAbil {
    /// Refers an item type involved in the relation.
    pub item_id: ItemId,
    /// Refers a fighter ability involved in the relation.
    pub abil_id: AbilId,
    /// Defines cooldown of the ability in seconds.
    pub cooldown: Option<AttrVal>,
    /// Defines how many times the ability can be used before the fighter has to rearm.
    pub charge_count: Option<Amount>,
    /// Defines how long each charge of the ability takes to rearm, in seconds.
    pub charge_rearm_time: Option<AttrVal>,
}
impl EItemFighterAbil {
    /// Makes a new EVE item-ability relation out of passed data.
    pub fn new(
        item_id: ItemId,
        abil_id: AbilId,
        cooldown: Option<AttrVal>,
        charge_count: Option<Amount>,
        charge_rearm_time: Option<AttrVal>,
    ) -> Self {
        Self {
            item_id,
            abil_id,
            cooldown,
            charge_count,
            charge_rearm_time,
        }
    }
}
impl Named for EItemFighterAbil {
    fn get_name() -> &'static str {
        "EItemFighterAbil"
    }
}
