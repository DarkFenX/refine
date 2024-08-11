use crate::{
    cmd::{change_item, HCmdResp},
    shared::HState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: rc::EItemId,
    state: HState,
}
impl HAddDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::SolDroneInfo, HExecError> {
        let core_drone = match core_sol.add_drone(*fit_id, self.type_id, (&self.state).into()) {
            Ok(core_drone) => core_drone,
            Err(error) => {
                return Err(match error {
                    rc::err::AddDroneError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                    rc::err::AddDroneError::ItemIdAllocFailed(e) => HExecError::ItemCapacityReached(e),
                })
            }
        };
        Ok(core_drone)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeDroneCmd,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
