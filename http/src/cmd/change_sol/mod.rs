pub(in crate::cmd) use fit::{HChangeFitCmd, HDeleteFitCmd};
pub(in crate::cmd) use fleet::{HAddFleetCmd, HChangeFleetCmd, HDeleteFleetCmd};
pub(in crate::cmd) use item::HRemoveItemCmd;
pub(in crate::cmd) use item_autocharge::HChangeAutochargeCmd;
pub(in crate::cmd) use item_booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(in crate::cmd) use item_character::{HChangeCharacterCmd, HRemoveCharacterCmd, HSetCharacterCmd};
pub(in crate::cmd) use item_charge::HChangeChargeCmd;
pub(in crate::cmd) use item_drone::{HAddDroneCmd, HChangeDroneCmd};
pub(in crate::cmd) use item_fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(in crate::cmd) use item_fw_effect::{HAddFwEffectCmd, HChangeFwEffectCmd};
pub(in crate::cmd) use item_implant::{HAddImplantCmd, HChangeImplantCmd};
pub(in crate::cmd) use item_module::{HAddModuleCmd, HChangeModuleCmd};
pub(in crate::cmd) use item_proj_effect::{HAddProjEffectCmd, HChangeProjEffectCmd};
pub(in crate::cmd) use item_rig::{HAddRigCmd, HChangeRigCmd};
pub(in crate::cmd) use item_service::{HAddServiceCmd, HChangeServiceCmd};
pub(in crate::cmd) use item_ship::{HChangeShipCmd, HRemoveShipCmd, HSetShipCmd};
pub(in crate::cmd) use item_skill::{HAddSkillCmd, HChangeSkillCmd};
pub(in crate::cmd) use item_stance::{HChangeStanceCmd, HRemoveStanceCmd, HSetStanceCmd};
pub(in crate::cmd) use item_subsystem::{HAddSubsystemCmd, HChangeSubsystemCmd};
pub(in crate::cmd) use item_sw_effect::{HAddSwEffectCmd, HChangeSwEffectCmd};
pub(in crate::cmd) use sol::HChangeSolCmd;

use crate::{
    cmd::{HAddFitCmd, HCmdResp},
    util::HExecError,
};

mod fit;
mod fleet;
mod item;
mod item_autocharge;
mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_proj_effect;
mod item_rig;
mod item_service;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_subsystem;
mod item_sw_effect;
mod sol;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeSolCommand {
    // Solar system
    ChangeSol(HChangeSolCmd),
    // Fleet
    AddFleet(HAddFleetCmd),
    ChangeFleet(HChangeFleetCmd),
    DeleteFleet(HDeleteFleetCmd),
    // Fit
    AddFit(HAddFitCmd),
    ChangeFit(HChangeFitCmd),
    DeleteFit(HDeleteFitCmd),
    // Item
    RemoveItem(HRemoveItemCmd),
    // Item - autocharge
    ChangeAutocharge(HChangeAutochargeCmd),
    // Item - booster
    AddBooster(HAddBoosterCmd),
    ChangeBooster(HChangeBoosterCmd),
    // Item - character
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    RemoveCharacter(HRemoveCharacterCmd),
    // Item - charge
    ChangeCharge(HChangeChargeCmd),
    // Item - drone
    AddDrone(HAddDroneCmd),
    ChangeDrone(HChangeDroneCmd),
    // Item - fighter
    AddFighter(HAddFighterCmd),
    ChangeFighter(HChangeFighterCmd),
    // Item - fit-wide effect
    AddFwEffect(HAddFwEffectCmd),
    ChangeFwEffect(HChangeFwEffectCmd),
    // Item - implant
    AddImplant(HAddImplantCmd),
    ChangeImplant(HChangeImplantCmd),
    // Item - module
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    // Item - projected effect
    AddProjEffect(HAddProjEffectCmd),
    ChangeProjEffect(HChangeProjEffectCmd),
    // Item - rig
    AddRig(HAddRigCmd),
    ChangeRig(HChangeRigCmd),
    // Item - service
    AddService(HAddServiceCmd),
    ChangeService(HChangeServiceCmd),
    // Item - ship
    SetShip(HSetShipCmd),
    ChangeShip(HChangeShipCmd),
    RemoveShip(HRemoveShipCmd),
    // Item - skill
    AddSkill(HAddSkillCmd),
    ChangeSkill(HChangeSkillCmd),
    // Item - stance
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    RemoveStance(HRemoveStanceCmd),
    // Item - subsystem
    AddSubsystem(HAddSubsystemCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
    // Item - system-wide effect
    AddSwEffect(HAddSwEffectCmd),
    ChangeSwEffect(HChangeSwEffectCmd),
}
impl HChangeSolCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            // Solar system
            Self::ChangeSol(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Fleet
            Self::AddFleet(cmd) => Ok(cmd.execute(core_sol).into()),
            Self::ChangeFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::DeleteFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Fit
            Self::AddFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::DeleteFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
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
            Self::RemoveCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
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
            Self::AddProjEffect(cmd) => Ok(cmd.execute(core_sol).into()),
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
            Self::RemoveShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => Ok(cmd.execute(core_sol).into()),
            Self::ChangeSwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
