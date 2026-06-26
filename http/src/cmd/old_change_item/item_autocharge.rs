use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeAutochargeCmd {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeAutochargeCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_autocharge = core_sol.get_autocharge_mut(item_id).map_err(|error| match error {
            rc::err::GetAutochargeError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetAutochargeError::ItemIsNotAutocharge(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = self.state {
            core_autocharge.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_autocharge);
        }
        Ok(HItemIdsResp::from_core_autocharge(core_autocharge))
    }
}
