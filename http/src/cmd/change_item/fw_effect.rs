use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFwEffectCmd {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeFwEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fw_effect = core_sol.get_fw_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetFwEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetFwEffectError::ItemIsNotFwEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = self.state {
            core_fw_effect.set_state(state);
        }
        apply_effect_modes(&mut core_fw_effect, &self.effect_modes);
        Ok(core_fw_effect.into())
    }
}
