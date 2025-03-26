use crate::{
    ed::{EAttrVal, EBuffId, EItemId},
    util::Named,
};

/// Space component data attached to an EVE item..
pub struct EItemSpaceComp {
    /// Refers an item type this data is attached to.
    pub item_id: EItemId,
    /// Buffs applicable to a filtered set of pilots in a solar system.
    pub system_emitter_buffs: Vec<EItemSpaceCompBuff>,
    /// Buffs applicable to entities close to effect carrier.
    pub proxy_effect_buffs: Vec<EItemSpaceCompBuff>,
    /// Buffs applicable to entities close to effect carrier when something triggers it.
    pub proxy_trigger_buffs: Vec<EItemSpaceCompBuff>,
    /// Buffs applicable to linked ship.
    pub ship_link_buffs: Vec<EItemSpaceCompBuff>,
}
impl EItemSpaceComp {
    pub(crate) fn has_buffs(&self) -> bool {
        !self.system_emitter_buffs.is_empty()
            || !self.proxy_effect_buffs.is_empty()
            || !self.proxy_trigger_buffs.is_empty()
            || !self.ship_link_buffs.is_empty()
    }
}
impl Named for EItemSpaceComp {
    fn get_name() -> &'static str {
        "EItemSpaceComp"
    }
}

/// Info about a space component buff.
pub struct EItemSpaceCompBuff {
    /// Buff ID to use.
    pub id: EBuffId,
    /// Buff modification strength.
    pub value: EAttrVal,
}
