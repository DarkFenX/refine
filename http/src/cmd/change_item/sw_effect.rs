use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSwEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_sw_effect = core_sol.get_sw_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetSwEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSwEffectError::ItemIsNotSwEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        apply_effect_modes(&mut core_sw_effect, &self.effect_modes);
        Ok(core_sw_effect.into())
    }
}
