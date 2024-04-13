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
pub(in crate::cmd) use item_ship::{HChangeShipCmd, HChangeShipViaFitIdCmd, HChangeShipViaItemIdCmd, HSetShipCmd};
pub(in crate::cmd) use item_skill::{HAddSkillCmd, HChangeSkillCmd};
pub(in crate::cmd) use item_stance::{
    HChangeStanceCmd, HChangeStanceViaFitIdCmd, HChangeStanceViaItemIdCmd, HSetStanceCmd,
};
pub(in crate::cmd) use item_structure::{
    HChangeStructureCmd, HChangeStructureViaFitIdCmd, HChangeStructureViaItemIdCmd, HSetStructureCmd,
};
pub(in crate::cmd) use item_subsystem::{HAddSubsystemCmd, HChangeSubsystemCmd};

use crate::cmd::HCmdResp;

mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_rig;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_structure;
mod item_subsystem;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeFitCommand {
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    AddSkill(HAddSkillCmd),
    ChangeSkill(HChangeSkillCmd),
    AddImplant(HAddImplantCmd),
    ChangeImplant(HChangeImplantCmd),
    AddBooster(HAddBoosterCmd),
    ChangeBooster(HChangeBoosterCmd),
    SetShip(HSetShipCmd),
    ChangeShip(HChangeShipCmd),
    SetStructure(HSetStructureCmd),
    ChangeStructure(HChangeStructureCmd),
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    AddSubsystem(HAddSubsystemCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    AddRig(HAddRigCmd),
    ChangeRig(HChangeRigCmd),
    AddDrone(HAddDroneCmd),
    ChangeDrone(HChangeDroneCmd),
    AddFighter(HAddFighterCmd),
    ChangeFighter(HChangeFighterCmd),
    ChangeCharge(HChangeChargeCmd),
    AddFwEffect(HAddFwEffectCmd),
    ChangeFwEffect(HChangeFwEffectCmd),
}
impl HChangeFitCommand {
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        match self {
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeCharacter(cmd) => cmd.execute(core_ss, fit_id),
            Self::AddSkill(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeSkill(cmd) => cmd.execute(core_ss),
            Self::AddImplant(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeImplant(cmd) => cmd.execute(core_ss),
            Self::AddBooster(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeBooster(cmd) => cmd.execute(core_ss),
            Self::SetShip(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeShip(cmd) => cmd.execute(core_ss, fit_id),
            Self::SetStructure(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeStructure(cmd) => cmd.execute(core_ss, fit_id),
            Self::SetStance(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeStance(cmd) => cmd.execute(core_ss, fit_id),
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeSubsystem(cmd) => cmd.execute(core_ss),
            Self::AddModule(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeModule(cmd) => cmd.execute(core_ss),
            Self::AddRig(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeRig(cmd) => cmd.execute(core_ss),
            Self::AddDrone(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeDrone(cmd) => cmd.execute(core_ss),
            Self::AddFighter(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeFighter(cmd) => cmd.execute(core_ss),
            Self::ChangeCharge(cmd) => cmd.execute(core_ss),
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_ss, fit_id)?.into()),
            Self::ChangeFwEffect(cmd) => cmd.execute(core_ss),
        }
    }
}
