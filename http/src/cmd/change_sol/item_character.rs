use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{HItemIdsResp, change_fit},
    util::HExecError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Setting
////////////////////////////////////////////////////////////////////////////////////////////////////
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HSetCharacterCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HSetCharacterCmd,
}
impl HSetCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeCharacterCmd {
    ViaItemId(HChangeCharacterViaItemIdCmd),
    ViaFitId(HChangeCharacterViaFitIdCmd),
}
impl HChangeCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Changing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
pub(crate) struct HChangeCharacterViaItemIdCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeCharacterViaItemIdCmd,
}
impl HChangeCharacterViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeCharacterViaFitIdCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeCharacterViaFitIdCmd,
}
impl HChangeCharacterViaFitIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Removing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HUnsetCharacterCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HUnsetCharacterCmd,
}
impl HUnsetCharacterCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}
