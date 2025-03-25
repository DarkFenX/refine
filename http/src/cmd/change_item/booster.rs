use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, HSideEffectMap, apply_effect_modes, apply_side_effects},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeBoosterCmd {
    state: Option<bool>,
    side_effects: Option<HSideEffectMap>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeBoosterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = self.state {
            if let Err(error) = core_sol.set_booster_state(item_id, state) {
                return Err(match error {
                    rc::err::SetBoosterStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetBoosterStateError::ItemIsNotBooster(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        apply_side_effects(core_sol, item_id, &self.side_effects)?;
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
