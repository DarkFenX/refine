use std::collections::HashMap;

use rc::ItemMutCommon;

use crate::shared::{HEffectId, HEffectMode};

pub(in crate::cmd) type HEffectModeMap = HashMap<HEffectId, HEffectMode>;

pub(in crate::cmd) fn apply_effect_modes(core_item: &mut impl ItemMutCommon, effect_modes: &Option<HEffectModeMap>) {
    if let Some(mode_map) = effect_modes
        && !mode_map.is_empty()
    {
        core_item.set_effect_modes(mode_map.iter().map(|(k, v)| (k.into(), v.into())));
    }
}
