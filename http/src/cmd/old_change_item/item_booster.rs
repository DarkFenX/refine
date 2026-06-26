use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HSideEffectMap},
    },
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    type_id: Option<i32>,
    state: Option<bool>,
    side_effects: Option<HSideEffectMap>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_booster = core_sol.get_booster_mut(item_id).map_err(|error| match error {
            rc::err::GetBoosterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetBoosterError::ItemIsNotBooster(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_booster.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        if let Some(h_side_effects) = self.side_effects.as_ref() {
            h_side_effects.apply(&mut core_booster);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_booster);
        }
        Ok(HItemIdsResp::from_core_booster(core_booster))
    }
}
