use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddRigCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HAddRigCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_rig = core_fit.add_rig(self.type_id);
        if let Some(state) = self.state {
            core_rig.set_state(state);
        }
        Ok(core_rig.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeRigCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeRigCmd,
}
impl HChangeRigCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
