use crate::{
    cmd::{HCmdResp, change_item},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetStanceCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HSetStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::StanceInfo, HExecError> {
        let core_stance = match core_sol.set_fit_stance(*fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_stance) => core_stance,
            Err(error) => {
                return Err(match error {
                    rc::err::SetFitStanceError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        Ok(core_stance)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeStanceCmd {
    ViaItemId(HChangeStanceViaItemIdCmd),
    ViaFitId(HChangeStanceViaFitIdCmd),
}
impl HChangeStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol, fit_id),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeStanceCmd,
}
impl HChangeStanceViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeStanceCmd,
}
impl HChangeStanceViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        let item_id = match core_sol.get_fit_stance(fit_id) {
            Ok(core_stance) => match core_stance {
                Some(core_stance) => core_stance.id,
                None => return Err(HExecError::FitStanceNotFound(*fit_id)),
            },
            Err(error) => {
                return Err(match error {
                    rc::err::GetFitStanceError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        self.item_cmd.execute(core_sol, &item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HRemoveStanceCmd {}
impl HRemoveStanceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        if let Err(error) = core_sol.remove_fit_stance(fit_id) {
            return Err(match error {
                rc::err::RemoveFitStanceError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                rc::err::RemoveFitStanceError::FitHasNoStance(e) => HExecError::FitItemKindNotFound(e),
            });
        };
        Ok(())
    }
}
