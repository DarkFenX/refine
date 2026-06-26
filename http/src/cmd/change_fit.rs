use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic::{
            HAutochargeChangeCmdFCtxBIds, HAutochargeChangeCmdFCtxRIds, HBoosterAddCmdICtx, HBoosterChangeCmdFCtxBIds,
            HBoosterChangeCmdFCtxRIds, HCharacterChangeCmdICtx, HCharacterSetCmdICtx, HCharacterUnsetCmdICtx,
            HChargeChangeCmdFCtxBIds, HChargeChangeCmdFCtxRIds, HDroneAddCmdICtxBIds, HDroneAddCmdICtxRIds,
            HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds, HFighterAddCmdICtxBIds, HFighterAddCmdICtxRIds,
            HFighterChangeCmdFCtxBIds, HFighterChangeCmdFCtxRIds, HFwEffectAddCmdICtx, HFwEffectChangeCmdFCtxBIds,
            HFwEffectChangeCmdFCtxRIds, HImplantAddCmdICtx, HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds,
            HItemRemoveCmdFCtxBIds, HItemRemoveCmdFCtxRIds, HModuleAddCmdICtxBIds, HModuleAddCmdICtxRIds,
            HModuleChangeCmdFCtxBIds, HModuleChangeCmdFCtxRIds, HRigAddCmdICtx, HRigChangeCmdFCtxBIds,
            HRigChangeCmdFCtxRIds, HServiceAddCmdICtx, HServiceChangeCmdFCtxBIds, HServiceChangeCmdFCtxRIds,
            HShipChangeCmdICtx, HShipSetCmdICtx, HShipUnsetCmdICtx, HSkillAddCmdICtx, HSkillChangeCmdFCtxBIds,
            HSkillChangeCmdFCtxRIds, HStanceChangeCmdICtx, HStanceSetCmdICtx, HStanceUnsetCmdICtx,
            HSubsystemAddCmdICtx, HSubsystemChangeCmdFCtxBIds, HSubsystemChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HFitChangeCmdBIds {
    // Item
    RemoveItem(HItemRemoveCmdFCtxBIds),
    // Item - autocharge
    ChangeAutocharge(HAutochargeChangeCmdFCtxBIds),
    // Item - booster
    AddBooster(HBoosterAddCmdICtx),
    ChangeBooster(HBoosterChangeCmdFCtxBIds),
    // Item - character
    SetCharacter(HCharacterSetCmdICtx),
    ChangeCharacter(HCharacterChangeCmdICtx),
    UnsetCharacter(HCharacterUnsetCmdICtx),
    // Item - charge
    ChangeCharge(HChargeChangeCmdFCtxBIds),
    // Item - drone
    AddDrone(HDroneAddCmdICtxBIds),
    ChangeDrone(HDroneChangeCmdFCtxBIds),
    // Item - fighter
    AddFighter(HFighterAddCmdICtxBIds),
    ChangeFighter(HFighterChangeCmdFCtxBIds),
    // Item - fit-wide effect
    AddFwEffect(HFwEffectAddCmdICtx),
    ChangeFwEffect(HFwEffectChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
    // Item - module
    AddModule(HModuleAddCmdICtxBIds),
    ChangeModule(HModuleChangeCmdFCtxBIds),
    // Item - rig
    AddRig(HRigAddCmdICtx),
    ChangeRig(HRigChangeCmdFCtxBIds),
    // Item - service
    AddService(HServiceAddCmdICtx),
    ChangeService(HServiceChangeCmdFCtxBIds),
    // Item - ship
    SetShip(HShipSetCmdICtx),
    ChangeShip(HShipChangeCmdICtx),
    UnsetShip(HShipUnsetCmdICtx),
    // Item - skill
    AddSkill(HSkillAddCmdICtx),
    ChangeSkill(HSkillChangeCmdFCtxBIds),
    // Item - stance
    SetStance(HStanceSetCmdICtx),
    ChangeStance(HStanceChangeCmdICtx),
    UnsetStance(HStanceUnsetCmdICtx),
    // Item - subsystem
    AddSubsystem(HSubsystemAddCmdICtx),
    ChangeSubsystem(HSubsystemChangeCmdFCtxBIds),
}

pub(crate) enum HFitChangeCmdRIds {
    // Item
    RemoveItem(HItemRemoveCmdFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(HAutochargeChangeCmdFCtxRIds),
    // Item - booster
    AddBooster(HBoosterAddCmdICtx),
    ChangeBooster(HBoosterChangeCmdFCtxRIds),
    // Item - character
    SetCharacter(HCharacterSetCmdICtx),
    ChangeCharacter(HCharacterChangeCmdICtx),
    UnsetCharacter(HCharacterUnsetCmdICtx),
    // Item - charge
    ChangeCharge(HChargeChangeCmdFCtxRIds),
    // Item - drone
    AddDrone(HDroneAddCmdICtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - fighter
    AddFighter(HFighterAddCmdICtxRIds),
    ChangeFighter(HFighterChangeCmdFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(HFwEffectAddCmdICtx),
    ChangeFwEffect(HFwEffectChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdICtx),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
    // Item - module
    AddModule(HModuleAddCmdICtxRIds),
    ChangeModule(HModuleChangeCmdFCtxRIds),
    // Item - rig
    AddRig(HRigAddCmdICtx),
    ChangeRig(HRigChangeCmdFCtxRIds),
    // Item - service
    AddService(HServiceAddCmdICtx),
    ChangeService(HServiceChangeCmdFCtxRIds),
    // Item - ship
    SetShip(HShipSetCmdICtx),
    ChangeShip(HShipChangeCmdICtx),
    UnsetShip(HShipUnsetCmdICtx),
    // Item - skill
    AddSkill(HSkillAddCmdICtx),
    ChangeSkill(HSkillChangeCmdFCtxRIds),
    // Item - stance
    SetStance(HStanceSetCmdICtx),
    ChangeStance(HStanceChangeCmdICtx),
    UnsetStance(HStanceUnsetCmdICtx),
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
            // Item
            Self::RemoveItem(cmd) => HFitChangeCmdRIds::RemoveItem(cmd.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => HFitChangeCmdRIds::ChangeAutocharge(cmd.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => HFitChangeCmdRIds::AddBooster(cmd),
            Self::ChangeBooster(cmd) => HFitChangeCmdRIds::ChangeBooster(cmd.render(resps)?),
            // Item - character
            Self::SetCharacter(cmd) => HFitChangeCmdRIds::SetCharacter(cmd),
            Self::ChangeCharacter(cmd) => HFitChangeCmdRIds::ChangeCharacter(cmd),
            Self::UnsetCharacter(cmd) => HFitChangeCmdRIds::UnsetCharacter(cmd),
            // Item - charge
            Self::ChangeCharge(cmd) => HFitChangeCmdRIds::ChangeCharge(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => HFitChangeCmdRIds::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => HFitChangeCmdRIds::ChangeDrone(cmd.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => HFitChangeCmdRIds::AddFighter(cmd.render(resps)?),
            Self::ChangeFighter(cmd) => HFitChangeCmdRIds::ChangeFighter(cmd.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => HFitChangeCmdRIds::AddFwEffect(cmd),
            Self::ChangeFwEffect(cmd) => HFitChangeCmdRIds::ChangeFwEffect(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HFitChangeCmdRIds::AddImplant(cmd),
            Self::ChangeImplant(cmd) => HFitChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
            // Item - module
            Self::AddModule(cmd) => HFitChangeCmdRIds::AddModule(cmd.render(resps)?),
            Self::ChangeModule(cmd) => HFitChangeCmdRIds::ChangeModule(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => HFitChangeCmdRIds::AddRig(cmd),
            Self::ChangeRig(cmd) => HFitChangeCmdRIds::ChangeRig(cmd.render(resps)?),
            // Item - service
            Self::AddService(cmd) => HFitChangeCmdRIds::AddService(cmd),
            Self::ChangeService(cmd) => HFitChangeCmdRIds::ChangeService(cmd.render(resps)?),
            // Item - ship
            Self::SetShip(cmd) => HFitChangeCmdRIds::SetShip(cmd),
            Self::ChangeShip(cmd) => HFitChangeCmdRIds::ChangeShip(cmd),
            Self::UnsetShip(cmd) => HFitChangeCmdRIds::UnsetShip(cmd),
            // Item - skill
            Self::AddSkill(cmd) => HFitChangeCmdRIds::AddSkill(cmd),
            Self::ChangeSkill(cmd) => HFitChangeCmdRIds::ChangeSkill(cmd.render(resps)?),
            // Item - stance
            Self::SetStance(cmd) => HFitChangeCmdRIds::SetStance(cmd),
            Self::ChangeStance(cmd) => HFitChangeCmdRIds::ChangeStance(cmd),
            Self::UnsetStance(cmd) => HFitChangeCmdRIds::UnsetStance(cmd),
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
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute_via_fit_id(core_sol, fit_id)?.into()),
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - ship
            Self::SetShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute_via_fit_id(core_sol, fit_id)?.into()),
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute_via_fit_id(core_sol, fit_id)?.into()),
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
