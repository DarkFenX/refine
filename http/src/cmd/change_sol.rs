use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic_item::{
            HAutochargeChangeCmdFCtxBIds, HAutochargeChangeCmdFCtxRIds, HBoosterAddCmdFCtxBIds, HBoosterAddCmdFCtxRIds,
            HBoosterChangeCmdFCtxBIds, HBoosterChangeCmdFCtxRIds, HChargeChangeCmdFCtxBIds, HChargeChangeCmdFCtxRIds,
            HDroneAddCmdFCtxBIds, HDroneAddCmdFCtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds,
            HFighterAddCmdFCtxBIds, HFighterAddCmdFCtxRIds, HFighterChangeCmdFCtxBIds, HFighterChangeCmdFCtxRIds,
            HFwEffectAddCmdFCtxBIds, HFwEffectAddCmdFCtxRIds, HFwEffectChangeCmdFCtxBIds, HFwEffectChangeCmdFCtxRIds,
            HImplantAddCmdFCtxBIds, HImplantAddCmdFCtxRIds, HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds,
            HModuleAddCmdFCtxBIds, HModuleAddCmdFCtxRIds, HModuleChangeCmdFCtxBIds, HModuleChangeCmdFCtxRIds,
            HProjEffectAddCmdFCtxBIds, HProjEffectAddCmdFCtxRIds, HProjEffectChangeCmdFCtxBIds,
            HProjEffectChangeCmdFCtxRIds, HRigAddCmdFCtxBIds, HRigAddCmdFCtxRIds, HRigChangeCmdFCtxBIds,
            HRigChangeCmdFCtxRIds, HServiceAddCmdFCtxBIds, HServiceAddCmdFCtxRIds, HServiceChangeCmdFCtxBIds,
            HServiceChangeCmdFCtxRIds, HShipChangeCmdFHybridCtxBIds, HShipChangeCmdFHybridCtxRIds,
            HSkillAddCmdFCtxBIds, HSkillAddCmdFCtxRIds, HSkillChangeCmdFCtxBIds, HSkillChangeCmdFCtxRIds,
            HSubsystemAddCmdFCtxBIds, HSubsystemAddCmdFCtxRIds, HSubsystemChangeCmdFCtxBIds,
            HSubsystemChangeCmdFCtxRIds, HSwEffectAddCmdFCtx, HSwEffectChangeCmdFCtxBIds, HSwEffectChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSolChangeCmdBIds {
    // Item - autocharge
    ChangeAutocharge(HAutochargeChangeCmdFCtxBIds),
    // Item - booster
    AddBooster(HBoosterAddCmdFCtxBIds),
    ChangeBooster(HBoosterChangeCmdFCtxBIds),
    // Item - charge
    ChangeCharge(HChargeChangeCmdFCtxBIds),
    // Item - drone
    AddDrone(HDroneAddCmdFCtxBIds),
    ChangeDrone(HDroneChangeCmdFCtxBIds),
    // Item - fighter
    AddFighter(HFighterAddCmdFCtxBIds),
    ChangeFighter(HFighterChangeCmdFCtxBIds),
    // Item - fit-wide effect
    AddFwEffect(HFwEffectAddCmdFCtxBIds),
    ChangeFwEffect(HFwEffectChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdFCtxBIds),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
    // Item - module
    AddModule(HModuleAddCmdFCtxBIds),
    ChangeModule(HModuleChangeCmdFCtxBIds),
    // Item - projected effect
    AddProjEffect(HProjEffectAddCmdFCtxBIds),
    ChangeProjEffect(HProjEffectChangeCmdFCtxBIds),
    // Item - rig
    AddRig(HRigAddCmdFCtxBIds),
    ChangeRig(HRigChangeCmdFCtxBIds),
    // Item - service
    AddService(HServiceAddCmdFCtxBIds),
    ChangeService(HServiceChangeCmdFCtxBIds),
    // Item - ship
    ChangeShip(HShipChangeCmdFHybridCtxBIds),
    // Item - skill
    AddSkill(HSkillAddCmdFCtxBIds),
    ChangeSkill(HSkillChangeCmdFCtxBIds),
    // Item - subsystem
    AddSubsystem(HSubsystemAddCmdFCtxBIds),
    ChangeSubsystem(HSubsystemChangeCmdFCtxBIds),
    // Item - system-wide effect
    AddSwEffect(HSwEffectAddCmdFCtx),
    ChangeSwEffect(HSwEffectChangeCmdFCtxBIds),
}

pub(crate) enum HSolChangeCmdRIds {
    // Item - autocharge
    ChangeAutocharge(HAutochargeChangeCmdFCtxRIds),
    // Item - booster
    AddBooster(HBoosterAddCmdFCtxRIds),
    ChangeBooster(HBoosterChangeCmdFCtxRIds),
    // Item - charge
    ChangeCharge(HChargeChangeCmdFCtxRIds),
    // Item - drone
    AddDrone(HDroneAddCmdFCtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - fighter
    AddFighter(HFighterAddCmdFCtxRIds),
    ChangeFighter(HFighterChangeCmdFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(HFwEffectAddCmdFCtxRIds),
    ChangeFwEffect(HFwEffectChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdFCtxRIds),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
    // Item - module
    AddModule(HModuleAddCmdFCtxRIds),
    ChangeModule(HModuleChangeCmdFCtxRIds),
    // Item - projected effect
    AddProjEffect(HProjEffectAddCmdFCtxRIds),
    ChangeProjEffect(HProjEffectChangeCmdFCtxRIds),
    // Item - rig
    AddRig(HRigAddCmdFCtxRIds),
    ChangeRig(HRigChangeCmdFCtxRIds),
    // Item - service
    AddService(HServiceAddCmdFCtxRIds),
    ChangeService(HServiceChangeCmdFCtxRIds),
    // Item - ship
    ChangeShip(HShipChangeCmdFHybridCtxRIds),
    // Item - skill
    AddSkill(HSkillAddCmdFCtxRIds),
    ChangeSkill(HSkillChangeCmdFCtxRIds),
    // Item - subsystem
    AddSubsystem(HSubsystemAddCmdFCtxRIds),
    ChangeSubsystem(HSubsystemChangeCmdFCtxRIds),
    // Item - system-wide effect
    AddSwEffect(HSwEffectAddCmdFCtx),
    ChangeSwEffect(HSwEffectChangeCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolChangeCmdBIds {
    pub(crate) fn render(self, resps: &HCmdResps) -> Result<HSolChangeCmdRIds, HExecError> {
        Ok(match self {
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => HSolChangeCmdRIds::ChangeAutocharge(cmd.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => HSolChangeCmdRIds::AddBooster(cmd.render(resps)?),
            Self::ChangeBooster(cmd) => HSolChangeCmdRIds::ChangeBooster(cmd.render(resps)?),
            // Item - charge
            Self::ChangeCharge(cmd) => HSolChangeCmdRIds::ChangeCharge(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => HSolChangeCmdRIds::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => HSolChangeCmdRIds::ChangeDrone(cmd.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => HSolChangeCmdRIds::AddFighter(cmd.render(resps)?),
            Self::ChangeFighter(cmd) => HSolChangeCmdRIds::ChangeFighter(cmd.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => HSolChangeCmdRIds::AddFwEffect(cmd.render(resps)?),
            Self::ChangeFwEffect(cmd) => HSolChangeCmdRIds::ChangeFwEffect(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HSolChangeCmdRIds::AddImplant(cmd.render(resps)?),
            Self::ChangeImplant(cmd) => HSolChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
            // Item - module
            Self::AddModule(cmd) => HSolChangeCmdRIds::AddModule(cmd.render(resps)?),
            Self::ChangeModule(cmd) => HSolChangeCmdRIds::ChangeModule(cmd.render(resps)?),
            // Item - projected effect
            Self::AddProjEffect(cmd) => HSolChangeCmdRIds::AddProjEffect(cmd.render(resps)?),
            Self::ChangeProjEffect(cmd) => HSolChangeCmdRIds::ChangeProjEffect(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => HSolChangeCmdRIds::AddRig(cmd.render(resps)?),
            Self::ChangeRig(cmd) => HSolChangeCmdRIds::ChangeRig(cmd.render(resps)?),
            // Item - service
            Self::AddService(cmd) => HSolChangeCmdRIds::AddService(cmd.render(resps)?),
            Self::ChangeService(cmd) => HSolChangeCmdRIds::ChangeService(cmd.render(resps)?),
            // Item - ship
            Self::ChangeShip(cmd) => HSolChangeCmdRIds::ChangeShip(cmd.render(resps)?),
            // Item - skill
            Self::AddSkill(cmd) => HSolChangeCmdRIds::AddSkill(cmd.render(resps)?),
            Self::ChangeSkill(cmd) => HSolChangeCmdRIds::ChangeSkill(cmd.render(resps)?),
            // Item - subsystem
            Self::AddSubsystem(cmd) => HSolChangeCmdRIds::AddSubsystem(cmd.render(resps)?),
            Self::ChangeSubsystem(cmd) => HSolChangeCmdRIds::ChangeSubsystem(cmd.render(resps)?),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => HSolChangeCmdRIds::AddSwEffect(cmd),
            Self::ChangeSwEffect(cmd) => HSolChangeCmdRIds::ChangeSwEffect(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolChangeCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - projected effect
            Self::AddProjEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeProjEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - ship
            Self::ChangeShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => Ok(cmd.execute(core_sol).into()),
            Self::ChangeSwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
