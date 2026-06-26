use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic_item::{
            HBoosterAddCmdICtx, HBoosterChangeCmdFCtxBIds, HBoosterChangeCmdFCtxRIds, HDroneAddCmdICtxBIds,
            HDroneAddCmdICtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds, HImplantAddCmdICtx,
            HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds, HRigAddCmdICtx, HRigChangeCmdFCtxBIds,
            HRigChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HFitChangeCmdBIds {
    // Item - booster
    AddBooster(HBoosterAddCmdICtx),
    ChangeBooster(HBoosterChangeCmdFCtxBIds),
    // Item - drone
    AddDrone(HDroneAddCmdICtxBIds),
    ChangeDrone(HDroneChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
    // Item - rig
    AddRig(HRigAddCmdICtx),
    ChangeRig(HRigChangeCmdFCtxBIds),
}

pub(crate) enum HFitChangeCmdRIds {
    // Item - booster
    AddBooster(HBoosterAddCmdICtx),
    ChangeBooster(HBoosterChangeCmdFCtxRIds),
    // Item - drone
    AddDrone(HDroneAddCmdICtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
    // Item - rig
    AddRig(HRigAddCmdICtx),
    ChangeRig(HRigChangeCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitChangeCmdBIds {
    pub(crate) fn render(self, resps: &HCmdResps) -> Result<HFitChangeCmdRIds, HExecError> {
        Ok(match self {
            // Item - booster
            Self::AddBooster(cmd) => HFitChangeCmdRIds::AddBooster(cmd),
            Self::ChangeBooster(cmd) => HFitChangeCmdRIds::ChangeBooster(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => HFitChangeCmdRIds::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => HFitChangeCmdRIds::ChangeDrone(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HFitChangeCmdRIds::AddImplant(cmd),
            Self::ChangeImplant(cmd) => HFitChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => HFitChangeCmdRIds::AddRig(cmd),
            Self::ChangeRig(cmd) => HFitChangeCmdRIds::ChangeRig(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitChangeCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
