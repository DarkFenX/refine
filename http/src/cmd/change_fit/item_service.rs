use crate::{
    cmd::{HCmdResp, change_item},
    shared::HServiceState,
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddServiceCmd {
    type_id: rc::EItemId,
    state: Option<HServiceState>,
}
impl HAddServiceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::ServiceInfo, HExecError> {
        let core_service = match core_sol.add_service(
            *fit_id,
            self.type_id,
            self.state.as_ref().unwrap_or(&HServiceState::Online).into(),
        ) {
            Ok(core_service) => core_service,
            Err(error) => {
                return Err(match error {
                    rc::err::AddServiceError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        Ok(core_service)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeServiceCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeServiceCmd,
}
impl HChangeServiceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
