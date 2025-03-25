use crate::{
    cmd::{HCmdResp, change_item},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::ShipInfo, HExecError> {
        let core_ship = match core_sol.set_fit_ship(*fit_id, self.type_id, self.state.unwrap_or(true)) {
            Ok(core_ship) => core_ship,
            Err(error) => {
                return Err(match error {
                    rc::err::SetFitShipError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        Ok(core_ship)
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
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
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeShipCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeShipCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCmdResp, HExecError> {
        let item_id = match core_sol.get_fit_ship(fit_id) {
            Ok(core_ship) => match core_ship {
                Some(core_ship) => core_ship.id,
                None => return Err(HExecError::FitShipNotFound(*fit_id)),
            },
            Err(error) => {
                return Err(match error {
                    rc::err::GetFitShipError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        self.item_cmd.execute(core_sol, &item_id)
    }
}
