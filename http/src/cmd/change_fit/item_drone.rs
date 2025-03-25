use crate::{
    cmd::{HCmdResp, change_item, shared::HMutationOnAdd},
    shared::HMinionState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    type_id: rc::ItemTypeId,
    state: HMinionState,
    mutation: Option<HMutationOnAdd>,
}
impl HAddDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::DroneInfo, HExecError> {
        let core_drone = match core_sol.add_drone(
            *fit_id,
            self.type_id,
            (&self.state).into(),
            self.mutation.as_ref().map(|v| v.into()),
        ) {
            Ok(core_drone) => core_drone,
            Err(error) => {
                return Err(match error {
                    rc::err::AddDroneError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        Ok(core_drone)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeDroneCmd,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
