use std::collections::HashMap;

use crate::shared::HEffectMode;

pub(in crate::cmd) type HEffectModeMap = HashMap<rc::EEffectId, HEffectMode>;

pub(in crate::cmd) fn apply_effect_modes(
    core_ss: &mut rc::SolarSystem,
    item_id: &rc::SsItemId,
    effect_modes: &Option<HEffectModeMap>,
) -> rc::Result<()> {
    if let Some(mode_map) = &effect_modes {
        if !mode_map.is_empty() {
            core_ss.set_item_effect_modes(item_id, mode_map.iter().map(|(k, v)| (*k, v.into())))?;
        }
    }
    Ok(())
}
