use rc::ItemMutCommon;
use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::shared::HEffectMode;

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HEffectModeMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    data: Vec<(rc::EffectId, HEffectMode)>,
}

pub(in crate::cmd) fn apply_effect_modes(core_item: &mut impl ItemMutCommon, effect_modes: &Option<HEffectModeMap>) {
    if let Some(mode_map) = effect_modes
        && !mode_map.data.is_empty()
    {
        core_item.set_effect_modes(mode_map.data.iter().map(|(k, v)| (*k, v.into_core())));
    }
}
