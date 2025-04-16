use crate::{
    cmd::{
        HCmdResp,
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
    ) -> Result<HCmdResp, HExecError> {
        let mut core_implant = match core_sol.get_implant_mut(item_id) {
            Ok(core_implant) => core_implant,
            Err(error) => {
                return Err(match error {
                    rc::err::GetImplantError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::GetImplantError::ItemIsNotImplant(e) => HExecError::ItemKindMismatch(e),
                });
            }
        };
        if let Some(state) = self.state {
            core_implant = core_implant.set_state(state);
        };
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
