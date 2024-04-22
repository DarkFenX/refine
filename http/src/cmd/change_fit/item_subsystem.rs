use crate::cmd::{change_item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSubsystemCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddSubsystemCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> rc::Result<rc::SolSubsystemInfo> {
        core_sol.add_subsystem(*fit_id, self.type_id, self.state.unwrap_or(true))
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSubsystemCmd,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
