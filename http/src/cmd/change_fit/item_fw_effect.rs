use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFwEffectCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddFwEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::SolFwEffectInfo, HExecError> {
        let core_fw_effect = match core_sol.add_fw_effect(*fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_fw_effect) => core_fw_effect,
            Err(error) => {
                return Err(match error {
                    rc::err::AddFwEffectError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                })
            }
        };
        Ok(core_fw_effect)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFwEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeFwEffectCmd,
}
impl HChangeFwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
