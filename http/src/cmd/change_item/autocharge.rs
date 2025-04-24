use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
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
        apply_effect_modes(&mut core_autocharge, &self.effect_modes);
        Ok(core_autocharge.into())
    }
}
