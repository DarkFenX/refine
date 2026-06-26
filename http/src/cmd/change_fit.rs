use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic_item::{
            HDroneAddCmdICtxBIds, HDroneAddCmdICtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds,
            HImplantAddCmdICtx, HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HFitChangeCmdBIds {
    // Item - drone
    AddDrone(HDroneAddCmdICtxBIds),
    ChangeDrone(HDroneChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
}

pub(crate) enum HFitChangeCmdRIds {
    // Item - drone
    AddDrone(HDroneAddCmdICtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitChangeCmdBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HFitChangeCmdRIds, HExecError> {
        Ok(match self {
            // Item - drone
            Self::AddDrone(cmd) => HFitChangeCmdRIds::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => HFitChangeCmdRIds::ChangeDrone(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HFitChangeCmdRIds::AddImplant(cmd),
            Self::ChangeImplant(cmd) => HFitChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitChangeCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
