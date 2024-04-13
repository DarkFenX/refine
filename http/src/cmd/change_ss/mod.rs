pub(in crate::cmd) use fit::{HCreateFitCmd, HDeleteFitCmd};
pub(in crate::cmd) use fleet::{HCreateFleetCmd, HDeleteFleetCmd};
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

mod fit;
mod fleet;
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
pub(crate) enum HChangeSsCommand {
    // Fleet commands
    CreateFleet(HCreateFleetCmd),
    DeleteFleet(HDeleteFleetCmd),
    // Fit commands
    CreateFit(HCreateFitCmd),
    DeleteFit(HDeleteFitCmd),
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
impl HChangeSsCommand {
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            // Fleet commands
            Self::CreateFleet(cmd) => cmd.execute(core_ss),
            Self::DeleteFleet(cmd) => cmd.execute(core_ss),
            // Fit commands
            Self::CreateFit(cmd) => cmd.execute(core_ss),
            Self::DeleteFit(cmd) => cmd.execute(core_ss),
            // Item commands
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeCharacter(cmd) => cmd.execute(core_ss),
            Self::AddSkill(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeSkill(cmd) => cmd.execute(core_ss),
            Self::AddImplant(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeImplant(cmd) => cmd.execute(core_ss),
            Self::AddBooster(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeBooster(cmd) => cmd.execute(core_ss),
            Self::SetShip(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeShip(cmd) => cmd.execute(core_ss),
            Self::SetStructure(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeStructure(cmd) => cmd.execute(core_ss),
            Self::SetStance(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeStance(cmd) => cmd.execute(core_ss),
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeSubsystem(cmd) => cmd.execute(core_ss),
            Self::AddModule(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeModule(cmd) => cmd.execute(core_ss),
            Self::AddRig(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeRig(cmd) => cmd.execute(core_ss),
            Self::AddDrone(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeDrone(cmd) => cmd.execute(core_ss),
            Self::AddFighter(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeFighter(cmd) => cmd.execute(core_ss),
            Self::ChangeCharge(cmd) => cmd.execute(core_ss),
            Self::AddSwEffect(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeSwEffect(cmd) => cmd.execute(core_ss),
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeFwEffect(cmd) => cmd.execute(core_ss),
            Self::AddProjEffect(cmd) => Ok(cmd.execute(core_ss)?.into()),
            Self::ChangeProjEffect(cmd) => cmd.execute(core_ss),
        }
    }
}
