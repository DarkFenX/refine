use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic_item::{
            HBoosterAddCmdICtx, HBoosterChangeCmdFCtxBIds, HBoosterChangeCmdFCtxRIds, HDroneAddCmdICtxBIds,
            HDroneAddCmdICtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds, HFwEffectAddCmdICtx,
            HFwEffectChangeCmdFCtxBIds, HFwEffectChangeCmdFCtxRIds, HImplantAddCmdICtx, HImplantChangeCmdFCtxBIds,
            HImplantChangeCmdFCtxRIds, HRigAddCmdICtx, HRigChangeCmdFCtxBIds, HRigChangeCmdFCtxRIds,
            HServiceAddCmdICtx, HServiceChangeCmdFCtxBIds, HServiceChangeCmdFCtxRIds, HSkillAddCmdICtx,
            HSkillChangeCmdFCtxBIds, HSkillChangeCmdFCtxRIds, HSubsystemAddCmdICtx, HSubsystemChangeCmdFCtxBIds,
            HSubsystemChangeCmdFCtxRIds,
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
    // Item - fit-wide effect
    AddFwEffect(HFwEffectAddCmdICtx),
    ChangeFwEffect(HFwEffectChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
    // Item - rig
    AddRig(HRigAddCmdICtx),
    ChangeRig(HRigChangeCmdFCtxBIds),
    // Item - service
    AddService(HServiceAddCmdICtx),
    ChangeService(HServiceChangeCmdFCtxBIds),
    // Item - skill
    AddSkill(HSkillAddCmdICtx),
    ChangeSkill(HSkillChangeCmdFCtxBIds),
    // Item - subsystem
    AddSubsystem(HSubsystemAddCmdICtx),
    ChangeSubsystem(HSubsystemChangeCmdFCtxBIds),
}

pub(crate) enum HFitChangeCmdRIds {
    // Item - booster
    AddBooster(HBoosterAddCmdICtx),
    ChangeBooster(HBoosterChangeCmdFCtxRIds),
    // Item - drone
    AddDrone(HDroneAddCmdICtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(HFwEffectAddCmdICtx),
    ChangeFwEffect(HFwEffectChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
    // Item - rig
    AddRig(HRigAddCmdICtx),
    ChangeRig(HRigChangeCmdFCtxRIds),
    // Item - service
    AddService(HServiceAddCmdICtx),
    ChangeService(HServiceChangeCmdFCtxRIds),
    // Item - skill
    AddSkill(HSkillAddCmdICtx),
    ChangeSkill(HSkillChangeCmdFCtxRIds),
    // Item - subsystem
    AddSubsystem(HSubsystemAddCmdICtx),
    ChangeSubsystem(HSubsystemChangeCmdFCtxRIds),
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
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => HFitChangeCmdRIds::AddFwEffect(cmd),
            Self::ChangeFwEffect(cmd) => HFitChangeCmdRIds::ChangeFwEffect(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HFitChangeCmdRIds::AddImplant(cmd),
            Self::ChangeImplant(cmd) => HFitChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => HFitChangeCmdRIds::AddRig(cmd),
            Self::ChangeRig(cmd) => HFitChangeCmdRIds::ChangeRig(cmd.render(resps)?),
            // Item - service
            Self::AddService(cmd) => HFitChangeCmdRIds::AddService(cmd),
            Self::ChangeService(cmd) => HFitChangeCmdRIds::ChangeService(cmd.render(resps)?),
            // Item - skill
            Self::AddSkill(cmd) => HFitChangeCmdRIds::AddSkill(cmd),
            Self::ChangeSkill(cmd) => HFitChangeCmdRIds::ChangeSkill(cmd.render(resps)?),
            // Item - subsystem
            Self::AddSubsystem(cmd) => HFitChangeCmdRIds::AddSubsystem(cmd),
            Self::ChangeSubsystem(cmd) => HFitChangeCmdRIds::ChangeSubsystem(cmd.render(resps)?),
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
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
