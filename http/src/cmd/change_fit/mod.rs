pub(in crate::cmd) use fit::HChangeFitCmd;
pub(in crate::cmd) use item_autocharge::HChangeAutochargeCmd;
pub(in crate::cmd) use item_booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(in crate::cmd) use item_character::{
    HChangeCharacterCmd, HChangeCharacterViaFitIdCmd, HChangeCharacterViaItemIdCmd, HSetCharacterCmd,
};
pub(in crate::cmd) use item_charge::HChangeChargeCmd;
pub(in crate::cmd) use item_drone::{HAddDroneCmd, HChangeDroneCmd};
pub(in crate::cmd) use item_fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(in crate::cmd) use item_fw_effect::{HAddFwEffectCmd, HChangeFwEffectCmd};
pub(in crate::cmd) use item_implant::{HAddImplantCmd, HChangeImplantCmd};
pub(in crate::cmd) use item_module::{HAddModuleCmd, HChangeModuleCmd};
pub(in crate::cmd) use item_rig::{HAddRigCmd, HChangeRigCmd};
pub(in crate::cmd) use item_service::{HAddServiceCmd, HChangeServiceCmd};
pub(in crate::cmd) use item_ship::{HChangeShipCmd, HChangeShipViaFitIdCmd, HChangeShipViaItemIdCmd, HSetShipCmd};
pub(in crate::cmd) use item_skill::{HAddSkillCmd, HChangeSkillCmd};
pub(in crate::cmd) use item_stance::{
    HChangeStanceCmd, HChangeStanceViaFitIdCmd, HChangeStanceViaItemIdCmd, HSetStanceCmd,
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
    ChangeFit(HChangeFitCmd),
    // Item commands
    ChangeAutocharge(HChangeAutochargeCmd),
    AddBooster(HAddBoosterCmd),
    ChangeBooster(HChangeBoosterCmd),
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    ChangeCharge(HChangeChargeCmd),
    AddDrone(HAddDroneCmd),
    ChangeDrone(HChangeDroneCmd),
    AddFighter(HAddFighterCmd),
    ChangeFighter(HChangeFighterCmd),
    AddFwEffect(HAddFwEffectCmd),
    ChangeFwEffect(HChangeFwEffectCmd),
    AddImplant(HAddImplantCmd),
    ChangeImplant(HChangeImplantCmd),
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    AddRig(HAddRigCmd),
    ChangeRig(HChangeRigCmd),
    AddService(HAddServiceCmd),
    ChangeService(HChangeServiceCmd),
    SetShip(HSetShipCmd),
    ChangeShip(HChangeShipCmd),
    AddSkill(HAddSkillCmd),
    ChangeSkill(HChangeSkillCmd),
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    AddSubsystem(HAddSubsystemCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
}
impl HChangeFitCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            Self::ChangeFit(cmd) => cmd.execute(core_sol, fit_id),
            // Item commands
            Self::ChangeAutocharge(cmd) => cmd.execute(core_sol),
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeBooster(cmd) => cmd.execute(core_sol),
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeCharacter(cmd) => cmd.execute(core_sol, fit_id),
            Self::ChangeCharge(cmd) => cmd.execute(core_sol),
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeDrone(cmd) => cmd.execute(core_sol),
            Self::AddFighter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFighter(cmd) => cmd.execute(core_sol),
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFwEffect(cmd) => cmd.execute(core_sol),
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeImplant(cmd) => cmd.execute(core_sol),
            Self::AddModule(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeModule(cmd) => cmd.execute(core_sol),
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeRig(cmd) => cmd.execute(core_sol),
            Self::AddService(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeService(cmd) => cmd.execute(core_sol),
            Self::SetShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeShip(cmd) => cmd.execute(core_sol, fit_id),
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSkill(cmd) => cmd.execute(core_sol),
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeStance(cmd) => cmd.execute(core_sol, fit_id),
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSubsystem(cmd) => cmd.execute(core_sol),
        }
    }
}
