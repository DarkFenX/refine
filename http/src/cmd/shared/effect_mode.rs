use std::collections::HashMap;

use crate::{shared::HEffectMode, util::HExecError};

pub(in crate::cmd) type HEffectModeMap = HashMap<rc::EEffectId, HEffectMode>;

pub(in crate::cmd) fn apply_effect_modes(
    core_sol: &mut rc::SolarSystem,
    item_id: &rc::SolItemId,
    effect_modes: &Option<HEffectModeMap>,
) -> Result<(), HExecError> {
    if let Some(mode_map) = effect_modes {
        if !mode_map.is_empty() {
            if let Err(error) = core_sol.set_item_effect_modes(item_id, mode_map.iter().map(|(k, v)| (*k, v.into()))) {
                return Err(match error {
                    rc::err::SetItemEffectModesError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                });
            }
        }
    }
    Ok(())
}
