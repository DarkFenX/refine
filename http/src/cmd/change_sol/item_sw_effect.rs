use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSwEffectCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<rc::SolSwEffectInfo, HExecError> {
        let sw_effect = match core_sol.add_sw_effect(self.type_id, self.state.unwrap_or(true)) {
            Ok(sw_effect) => sw_effect,
            Err(error) => {
                return Err(match error {
                    rc::err::AddSwEffectError::ItemIdAllocFailed(e) => HExecError::ItemCapacityReached(e),
                })
            }
        };
        Ok(sw_effect)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSwEffectCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSwEffectCmd,
}
impl HChangeSwEffectCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
