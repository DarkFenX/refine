use crate::{
    cmd::{HCmdResp, change_item},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddImplantCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HAddImplantCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::ItemId, HExecError> {
        let mut core_implant = match core_sol.add_implant(fit_id, self.type_id) {
            Ok(core_implant) => core_implant,
            Err(error) => {
                return Err(match error {
                    rc::err::AddImplantError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        if let Some(state) = self.state {
            core_implant = core_implant.set_state(state);
        };
        Ok(core_implant.get_item_id())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeImplantCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeImplantCmd,
}
impl HChangeImplantCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
