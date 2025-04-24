use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    shared::HServiceState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddServiceCmd {
    type_id: rc::ItemTypeId,
    state: HServiceState,
}
impl HAddServiceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_service = core_fit.add_service(self.type_id, (&self.state).into());
        Ok(core_service.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeServiceCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeServiceCmd,
}
impl HChangeServiceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
