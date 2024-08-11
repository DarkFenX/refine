use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddProjEffectCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::SolProjEffectInfo, HExecError> {
        let proj_effect = match core_sol.add_proj_effect(self.type_id, self.state.unwrap_or(true)) {
            Ok(proj_effect) => proj_effect,
            Err(error) => {
                return Err(match error {
                    rc::err::AddProjEffectError::ItemIdAllocFailed(e) => HExecError::ItemCapacityReached(e),
                })
            }
        };
        Ok(proj_effect)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeProjEffectCmd,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
