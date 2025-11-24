use crate::{
    ed::{EAttrVal, EBuffId, EItemId, EItemListId},
    util::Named,
};

/// Space component data attached to an EVE item..
pub struct EItemSpaceComp {
    /// Refers an item type this data is attached to.
    pub item_id: EItemId,
    /// Buffs applicable to pilots in a solar system, with custom function as a filter.
    pub system_wide_buffs: Option<EItemSpaceCompBuffData>,
    /// Buffs applicable to pilots in a solar system, with custom function as a filter.
    pub system_emitter_buffs: Option<EItemSpaceCompBuffData>,
    /// Buffs applicable to entities close to effect carrier.
    pub proxy_effect_buffs: Option<EItemSpaceCompBuffData>,
    /// Buffs applicable to entities close to effect carrier when something triggers it.
    pub proxy_trigger_buffs: Option<EItemSpaceCompBuffData>,
    /// Buffs applicable to linked ship.
    pub ship_link_buffs: Option<EItemSpaceCompBuffData>,
}
impl EItemSpaceComp {
    pub(crate) fn iter_data(&self) -> impl Iterator<Item = &Option<EItemSpaceCompBuffData>> {
        [
            &self.system_wide_buffs,
            &self.system_emitter_buffs,
            &self.proxy_effect_buffs,
            &self.proxy_trigger_buffs,
            &self.ship_link_buffs,
        ]
        .into_iter()
    }
    pub(crate) fn has_buffs(&self) -> bool {
        for buff_data in self.iter_data().filter_map(|v| v.as_ref()) {
            if !buff_data.buffs.is_empty() {
                return true;
            }
        }
        false
    }
}
impl Named for EItemSpaceComp {
    fn get_name() -> &'static str {
        "EItemSpaceComp"
    }
}

/// Info about buffs attached to a space component.
pub struct EItemSpaceCompBuffData {
    /// List of buffs to apply.
    pub buffs: Vec<EItemSpaceCompBuff>,
    /// Buff modification strength.
    pub item_list_filter: Option<EItemListId>,
}

/// Info about one of space component buffs.
pub struct EItemSpaceCompBuff {
    /// Buff ID to use.
    pub id: EBuffId,
    /// Buff modification strength.
    pub value: EAttrVal,
}
