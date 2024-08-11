use crate::{
    cmd::{change_item, HCmdResp},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddImplantCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddImplantCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::SolImplantInfo, HExecError> {
        let core_implant = match core_sol.add_implant(*fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_implant) => core_implant,
            Err(error) => {
                return Err(match error {
                    rc::err::AddImplantError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                    rc::err::AddImplantError::ItemIdAllocFailed(e) => HExecError::ItemCapacityReached(e),
                })
            }
        };
        Ok(core_implant)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeImplantCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeImplantCmd,
}
impl HChangeImplantCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
