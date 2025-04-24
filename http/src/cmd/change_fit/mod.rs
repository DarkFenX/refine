pub(in crate::cmd) use fit::HChangeFitCmd;
pub(in crate::cmd) use item_autocharge::HChangeAutochargeCmd;
pub(in crate::cmd) use item_booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(in crate::cmd) use item_character::{
    HChangeCharacterCmd, HChangeCharacterViaFitIdCmd, HChangeCharacterViaItemIdCmd, HRemoveCharacterCmd,
    HSetCharacterCmd,
};
pub(in crate::cmd) use item_charge::HChangeChargeCmd;
pub(in crate::cmd) use item_drone::{HAddDroneCmd, HChangeDroneCmd};
pub(in crate::cmd) use item_fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(in crate::cmd) use item_fw_effect::{HAddFwEffectCmd, HChangeFwEffectCmd};
pub(in crate::cmd) use item_implant::{HAddImplantCmd, HChangeImplantCmd};
pub(in crate::cmd) use item_module::{HAddModuleCmd, HChangeModuleCmd};
pub(in crate::cmd) use item_rig::{HAddRigCmd, HChangeRigCmd};
pub(in crate::cmd) use item_service::{HAddServiceCmd, HChangeServiceCmd};
pub(in crate::cmd) use item_ship::{
    HChangeShipCmd, HChangeShipViaFitIdCmd, HChangeShipViaItemIdCmd, HRemoveShipCmd, HSetShipCmd,
};
pub(in crate::cmd) use item_skill::{HAddSkillCmd, HChangeSkillCmd};
pub(in crate::cmd) use item_stance::{
    HChangeStanceCmd, HChangeStanceViaFitIdCmd, HChangeStanceViaItemIdCmd, HRemoveStanceCmd, HSetStanceCmd,
};
pub(in crate::cmd) use item_subsystem::{HAddSubsystemCmd, HChangeSubsystemCmd};

use crate::{cmd::HCmdResp, util::HExecError};

mod fit;
mod item_autocharge;
mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_rig;
mod item_service;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_subsystem;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeFitCommand {
    // Fit
    ChangeFit(HChangeFitCmd),
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
}
impl HChangeFitCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::RemoveCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
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
            Self::ChangeShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::RemoveShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::RemoveStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
