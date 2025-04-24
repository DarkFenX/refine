use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSubsystemCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HAddSubsystemCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_subsystem = core_fit.add_subsystem(self.type_id);
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        }
        Ok(core_subsystem.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeSubsystemCmd,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
