use crate::{
    ed::{EAbilId, EAttrVal, ECount, EItemId},
    util::Named,
};

/// An EVE item type-fighter ability relation.
pub struct EItemFighterAbil {
    /// Refers an item type involved in the relation.
    pub item_id: EItemId,
    /// Refers a fighter ability involved in the relation.
    pub abil_id: EAbilId,
    /// Defines cooldown of the ability in seconds.
    pub cooldown: Option<EAttrVal>,
    /// Defines how many times the ability can be used before the fighter has to rearm.
    pub charge_count: Option<ECount>,
    /// Defines how long each charge of the ability takes to rearm, in seconds.
    pub charge_rearm_time: Option<EAttrVal>,
}
impl Named for EItemFighterAbil {
    fn get_name() -> &'static str {
        "EItemFighterAbil"
    }
}
