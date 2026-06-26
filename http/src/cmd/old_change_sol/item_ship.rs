use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, old_change_fit},
    util::HExecError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Setting
////////////////////////////////////////////////////////////////////////////////////////////////////
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HSetShipCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: old_change_fit::HSetShipCmd,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Changing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: old_change_fit::HChangeShipViaItemIdCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: old_change_fit::HChangeShipViaFitIdCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Removing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HUnsetShipCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: old_change_fit::HUnsetShipCmd,
}
impl HUnsetShipCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}
