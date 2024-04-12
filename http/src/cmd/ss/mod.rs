pub(in crate::cmd) use item_booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(in crate::cmd) use item_character::{HChangeCharacterCmd, HSetCharacterCmd};
pub(in crate::cmd) use item_charge::HChangeChargeCmd;
pub(in crate::cmd) use item_drone::{HAddDroneCmd, HChangeDroneCmd};
pub(in crate::cmd) use item_fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(in crate::cmd) use item_fw_effect::{HAddFwEffectCmd, HChangeFwEffectCmd};
pub(in crate::cmd) use item_implant::{HAddImplantCmd, HChangeImplantCmd};
pub(in crate::cmd) use item_module::{HAddModuleCmd, HChangeModuleCmd};
pub(in crate::cmd) use item_proj_effect::{HAddProjEffectCmd, HChangeProjEffectCmd};
pub(in crate::cmd) use item_rig::{HAddRigCmd, HChangeRigCmd};
pub(in crate::cmd) use item_ship::{HChangeShipCmd, HSetShipCmd};
pub(in crate::cmd) use item_skill::{HAddSkillCmd, HChangeSkillCmd};
pub(in crate::cmd) use item_stance::{HChangeStanceCmd, HSetStanceCmd};
pub(in crate::cmd) use item_structure::{HChangeStructureCmd, HSetStructureCmd};
pub(in crate::cmd) use item_subsystem::{HAddSubsystemCmd, HChangeSubsystemCmd};
pub(in crate::cmd) use item_sw_effect::{HAddSwEffectCmd, HChangeSwEffectCmd};

use crate::cmd::HCmdResp;

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
mod item_ship;
mod item_skill;
mod item_stance;
mod item_structure;
mod item_subsystem;
mod item_sw_effect;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSsCommand {
    // Item commands
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
    AddSwEffect(HAddSwEffectCmd),
    ChangeSwEffect(HChangeSwEffectCmd),
    AddFwEffect(HAddFwEffectCmd),
    ChangeFwEffect(HChangeFwEffectCmd),
    AddProjEffect(HAddProjEffectCmd),
    ChangeProjEffect(HChangeProjEffectCmd),
}
impl HSsCommand {
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            // Item commands
            Self::SetCharacter(cmd) => cmd.execute(core_ss),
            Self::ChangeCharacter(cmd) => cmd.execute(core_ss),
            Self::AddSkill(cmd) => cmd.execute(core_ss),
            Self::ChangeSkill(cmd) => cmd.execute(core_ss),
            Self::AddImplant(cmd) => cmd.execute(core_ss),
            Self::ChangeImplant(cmd) => cmd.execute(core_ss),
            Self::AddBooster(cmd) => cmd.execute(core_ss),
            Self::ChangeBooster(cmd) => cmd.execute(core_ss),
            Self::SetShip(cmd) => cmd.execute(core_ss),
            Self::ChangeShip(cmd) => cmd.execute(core_ss),
            Self::SetStructure(cmd) => cmd.execute(core_ss),
            Self::ChangeStructure(cmd) => cmd.execute(core_ss),
            Self::SetStance(cmd) => cmd.execute(core_ss),
            Self::ChangeStance(cmd) => cmd.execute(core_ss),
            Self::AddSubsystem(cmd) => cmd.execute(core_ss),
            Self::ChangeSubsystem(cmd) => cmd.execute(core_ss),
            Self::AddModule(cmd) => cmd.execute(core_ss),
            Self::ChangeModule(cmd) => cmd.execute(core_ss),
            Self::AddRig(cmd) => cmd.execute(core_ss),
            Self::ChangeRig(cmd) => cmd.execute(core_ss),
            Self::AddDrone(cmd) => cmd.execute(core_ss),
            Self::ChangeDrone(cmd) => cmd.execute(core_ss),
            Self::AddFighter(cmd) => cmd.execute(core_ss),
            Self::ChangeFighter(cmd) => cmd.execute(core_ss),
            Self::ChangeCharge(cmd) => cmd.execute(core_ss),
            Self::AddSwEffect(cmd) => cmd.execute(core_ss),
            Self::ChangeSwEffect(cmd) => cmd.execute(core_ss),
            Self::AddFwEffect(cmd) => cmd.execute(core_ss),
            Self::ChangeFwEffect(cmd) => cmd.execute(core_ss),
            Self::AddProjEffect(cmd) => cmd.execute(core_ss),
            Self::ChangeProjEffect(cmd) => cmd.execute(core_ss),
        }
    }
}
