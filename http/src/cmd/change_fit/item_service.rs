use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    shared::HServiceState,
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HAddServiceCmd {
    type_id: i32,
    state: HServiceState,
}
impl HAddServiceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        let core_state = self.state.into_core();
        let core_service = core_fit.add_service(core_type_id, core_state);
        Ok(core_service.into())
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeServiceCmd {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeServiceCmd,
}
impl HChangeServiceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
