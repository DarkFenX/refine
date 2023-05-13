use crate::{
    defines::{ReeFloat, ReeInt},
    util::Named,
};

/// An item type - fighter ability relation.
#[derive(Debug)]
pub struct ItemFighterAbil {
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers a fighter ability involved in the relation.
    pub abil_id: ReeInt,
    /// Defines cooldown of the ability in seconds.
    pub cooldown: Option<ReeFloat>,
    /// Defines how many times the ability can be used before the fighter has to rearm.
    pub charge_count: Option<ReeInt>,
    /// Defines how long each charge of the ability takes to rearm, in seconds.
    pub charge_rearm_time: Option<ReeFloat>,
}
impl ItemFighterAbil {
    /// Makes a new item-ability relation out of passed data.
    pub fn new(
        item_id: ReeInt,
        abil_id: ReeInt,
        cooldown: Option<ReeFloat>,
        charge_count: Option<ReeInt>,
        charge_rearm_time: Option<ReeFloat>,
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
impl Named for ItemFighterAbil {
    fn get_name() -> &'static str {
        "dh::ItemFighterAbil"
    }
}
