use serde_tuple::Serialize_tuple;

use crate::shared::HRearmMinion;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_fighter) struct HItemRearmMinionInfo {
    value: HRearmMinion,
    overridden: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemRearmMinionInfo {
    pub(in crate::info::item::item_fighter) fn from_core(core_rearm_minion: rc::ItemRearmMinionInfo) -> Self {
        Self {
            value: HRearmMinion::from_core(core_rearm_minion.value),
            overridden: core_rearm_minion.overridden,
        }
    }
}
