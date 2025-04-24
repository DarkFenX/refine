use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeImplantCmd {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeImplantCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_implant = core_sol.get_implant_mut(item_id).map_err(|error| match error {
            rc::err::GetImplantError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetImplantError::ItemIsNotImplant(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = self.state {
            core_implant.set_state(state);
        };
        apply_effect_modes(&mut core_implant, &self.effect_modes);
        Ok(core_implant.into())
    }
}
