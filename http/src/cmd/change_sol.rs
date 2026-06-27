use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic::{
            HAutochargeChangeCmdFCtxBIds, HAutochargeChangeCmdFCtxRIds, HBoosterAddCmdFCtxBIds, HBoosterAddCmdFCtxRIds,
            HBoosterChangeCmdFCtxBIds, HBoosterChangeCmdFCtxRIds, HCharacterChangeCmdFHybridCtxBIds,
            HCharacterChangeCmdFHybridCtxRIds, HCharacterSetCmdFCtxBIds, HCharacterSetCmdFCtxRIds,
            HCharacterUnsetCmdFCtxBIds, HCharacterUnsetCmdFCtxRIds, HChargeChangeCmdFCtxBIds, HChargeChangeCmdFCtxRIds,
            HDroneAddCmdFCtxBIds, HDroneAddCmdFCtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds,
            HFighterAddCmdFCtxBIds, HFighterAddCmdFCtxRIds, HFighterChangeCmdFCtxBIds, HFighterChangeCmdFCtxRIds,
            HFitChangeCmdFCtxBIds, HFitChangeCmdFCtxRIds, HFitRemoveCmdFCtxBIds, HFitRemoveCmdFCtxRIds,
            HFwEffectAddCmdFCtxBIds, HFwEffectAddCmdFCtxRIds, HFwEffectChangeCmdFCtxBIds, HFwEffectChangeCmdFCtxRIds,
            HImplantAddCmdFCtxBIds, HImplantAddCmdFCtxRIds, HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds,
            HItemRemoveCmdFCtxBIds, HItemRemoveCmdFCtxRIds, HModuleAddCmdFCtxBIds, HModuleAddCmdFCtxRIds,
            HModuleChangeCmdFCtxBIds, HModuleChangeCmdFCtxRIds, HProjEffectAddCmdFCtxBIds, HProjEffectAddCmdFCtxRIds,
            HProjEffectChangeCmdFCtxBIds, HProjEffectChangeCmdFCtxRIds, HRigAddCmdFCtxBIds, HRigAddCmdFCtxRIds,
            HRigChangeCmdFCtxBIds, HRigChangeCmdFCtxRIds, HServiceAddCmdFCtxBIds, HServiceAddCmdFCtxRIds,
            HServiceChangeCmdFCtxBIds, HServiceChangeCmdFCtxRIds, HShipChangeCmdFHybridCtxBIds,
            HShipChangeCmdFHybridCtxRIds, HShipSetCmdFCtxBIds, HShipSetCmdFCtxRIds, HShipUnsetCmdFCtxBIds,
            HShipUnsetCmdFCtxRIds, HSkillAddCmdFCtxBIds, HSkillAddCmdFCtxRIds, HSkillChangeCmdFCtxBIds,
            HSkillChangeCmdFCtxRIds, HStanceChangeCmdFHybridCtxBIds, HStanceChangeCmdFHybridCtxRIds,
            HStanceSetCmdFCtxBIds, HStanceSetCmdFCtxRIds, HStanceUnsetCmdFCtxBIds, HStanceUnsetCmdFCtxRIds,
            HSubsystemAddCmdFCtxBIds, HSubsystemAddCmdFCtxRIds, HSubsystemChangeCmdFCtxBIds,
            HSubsystemChangeCmdFCtxRIds, HSwEffectAddCmdFCtx, HSwEffectChangeCmdFCtxBIds, HSwEffectChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSolChangeCmdBIds {
    // Fit
    ChangeFit(HFitChangeCmdFCtxBIds),
    RemoveFit(HFitRemoveCmdFCtxBIds),
    // Item
    RemoveItem(HItemRemoveCmdFCtxBIds),
    // Item - autocharge
    ChangeAutocharge(HAutochargeChangeCmdFCtxBIds),
    // Item - booster
    AddBooster(HBoosterAddCmdFCtxBIds),
    ChangeBooster(HBoosterChangeCmdFCtxBIds),
    // Item - character
    SetCharacter(HCharacterSetCmdFCtxBIds),
    ChangeCharacter(HCharacterChangeCmdFHybridCtxBIds),
    UnsetCharacter(HCharacterUnsetCmdFCtxBIds),
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
    SetShip(HShipSetCmdFCtxBIds),
    ChangeShip(HShipChangeCmdFHybridCtxBIds),
    UnsetShip(HShipUnsetCmdFCtxBIds),
    // Item - skill
    AddSkill(HSkillAddCmdFCtxBIds),
    ChangeSkill(HSkillChangeCmdFCtxBIds),
    // Item - stance
    SetStance(HStanceSetCmdFCtxBIds),
    ChangeStance(HStanceChangeCmdFHybridCtxBIds),
    UnsetStance(HStanceUnsetCmdFCtxBIds),
    // Item - subsystem
    AddSubsystem(HSubsystemAddCmdFCtxBIds),
    ChangeSubsystem(HSubsystemChangeCmdFCtxBIds),
    // Item - system-wide effect
    AddSwEffect(HSwEffectAddCmdFCtx),
    ChangeSwEffect(HSwEffectChangeCmdFCtxBIds),
}

pub(crate) enum HSolChangeCmdRIds {
    // Fit
    ChangeFit(HFitChangeCmdFCtxRIds),
    RemoveFit(HFitRemoveCmdFCtxRIds),
    // Item
    RemoveItem(HItemRemoveCmdFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(HAutochargeChangeCmdFCtxRIds),
    // Item - booster
    AddBooster(HBoosterAddCmdFCtxRIds),
    ChangeBooster(HBoosterChangeCmdFCtxRIds),
    // Item - character
    SetCharacter(HCharacterSetCmdFCtxRIds),
    ChangeCharacter(HCharacterChangeCmdFHybridCtxRIds),
    UnsetCharacter(HCharacterUnsetCmdFCtxRIds),
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
    SetShip(HShipSetCmdFCtxRIds),
    ChangeShip(HShipChangeCmdFHybridCtxRIds),
    UnsetShip(HShipUnsetCmdFCtxRIds),
    // Item - skill
    AddSkill(HSkillAddCmdFCtxRIds),
    ChangeSkill(HSkillChangeCmdFCtxRIds),
    // Item - stance
    SetStance(HStanceSetCmdFCtxRIds),
    ChangeStance(HStanceChangeCmdFHybridCtxRIds),
    UnsetStance(HStanceUnsetCmdFCtxRIds),
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
            // Fit
            Self::ChangeFit(cmd) => HSolChangeCmdRIds::ChangeFit(cmd.render(resps)?),
            Self::RemoveFit(cmd) => HSolChangeCmdRIds::RemoveFit(cmd.render(resps)?),
            // Item
            Self::RemoveItem(cmd) => HSolChangeCmdRIds::RemoveItem(cmd.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => HSolChangeCmdRIds::ChangeAutocharge(cmd.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => HSolChangeCmdRIds::AddBooster(cmd.render(resps)?),
            Self::ChangeBooster(cmd) => HSolChangeCmdRIds::ChangeBooster(cmd.render(resps)?),
            // Item - character
            Self::SetCharacter(cmd) => HSolChangeCmdRIds::SetCharacter(cmd.render(resps)?),
            Self::ChangeCharacter(cmd) => HSolChangeCmdRIds::ChangeCharacter(cmd.render(resps)?),
            Self::UnsetCharacter(cmd) => HSolChangeCmdRIds::UnsetCharacter(cmd.render(resps)?),
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
            Self::SetShip(cmd) => HSolChangeCmdRIds::SetShip(cmd.render(resps)?),
            Self::ChangeShip(cmd) => HSolChangeCmdRIds::ChangeShip(cmd.render(resps)?),
            Self::UnsetShip(cmd) => HSolChangeCmdRIds::UnsetShip(cmd.render(resps)?),
            // Item - skill
            Self::AddSkill(cmd) => HSolChangeCmdRIds::AddSkill(cmd.render(resps)?),
            Self::ChangeSkill(cmd) => HSolChangeCmdRIds::ChangeSkill(cmd.render(resps)?),
            // Item - stance
            Self::SetStance(cmd) => HSolChangeCmdRIds::SetStance(cmd.render(resps)?),
            Self::ChangeStance(cmd) => HSolChangeCmdRIds::ChangeStance(cmd.render(resps)?),
            Self::UnsetStance(cmd) => HSolChangeCmdRIds::UnsetStance(cmd.render(resps)?),
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
            // Item
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
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
            Self::SetShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => Ok(cmd.execute(core_sol).into()),
            Self::ChangeSwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
