use std::collections::HashMap;

use crate::{shared::HEffectId, util::HExecError};

pub(in crate::cmd) type HSideEffectMap = HashMap<HEffectId, bool>;

pub(in crate::cmd) fn apply_side_effects(
    core_sol: &mut rc::SolarSystem,
    item_id: &rc::ItemId,
    side_effects: &Option<HSideEffectMap>,
) -> Result<(), HExecError> {
    if let Some(side_effect_map) = side_effects {
        for (effect_id, status) in side_effect_map.iter() {
            if let Err(error) = core_sol.set_booster_side_effect_state(item_id, &effect_id.into(), *status) {
                return Err(match error {
                    rc::err::SetBoosterSideEffectStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetBoosterSideEffectStateError::ItemIsNotBooster(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
    }
    Ok(())
}
