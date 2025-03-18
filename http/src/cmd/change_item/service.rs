use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeServiceCmd {
    state: Option<bool>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeServiceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = self.state {
            if let Err(error) = core_sol.set_service_state(item_id, state) {
                return Err(match error {
                    rc::err::SetServiceStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetServiceStateError::ItemIsNotService(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
