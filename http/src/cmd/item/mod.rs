pub(in crate::cmd) use booster::HChangeBoosterCmd;
pub(in crate::cmd) use character::HChangeCharacterCmd;
pub(in crate::cmd) use charge::HChangeChargeCmd;
pub(in crate::cmd) use drone::HChangeDroneCmd;
pub(in crate::cmd) use fighter::HChangeFighterCmd;
pub(in crate::cmd) use fw_effect::HChangeFwEffectCmd;
pub(in crate::cmd) use implant::HChangeImplantCmd;
pub(in crate::cmd) use module::HChangeModuleCmd;
pub(in crate::cmd) use proj_effect::HChangeProjEffectCmd;
pub(in crate::cmd) use rig::HChangeRigCmd;
pub(in crate::cmd) use ship::HChangeShipCmd;
pub(in crate::cmd) use skill::HChangeSkillCmd;
pub(in crate::cmd) use stance::HChangeStanceCmd;
pub(in crate::cmd) use structure::HChangeStructureCmd;
pub(in crate::cmd) use subsystem::HChangeSubsystemCmd;
pub(in crate::cmd) use sw_effect::HChangeSwEffectCmd;

use crate::cmd::HCmdResp;

mod booster;
mod character;
mod charge;
mod drone;
mod fighter;
mod fw_effect;
mod implant;
mod module;
mod proj_effect;
mod rig;
mod ship;
mod skill;
mod stance;
mod structure;
mod subsystem;
mod sw_effect;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemCommand {
    ChangeCharacter(HChangeCharacterCmd),
    ChangeSkill(HChangeSkillCmd),
    ChangeImplant(HChangeImplantCmd),
    ChangeBooster(HChangeBoosterCmd),
    ChangeShip(HChangeShipCmd),
    ChangeStructure(HChangeStructureCmd),
    ChangeStance(HChangeStanceCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
    ChangeModule(HChangeModuleCmd),
    ChangeRig(HChangeRigCmd),
    ChangeDrone(HChangeDroneCmd),
    ChangeFighter(HChangeFighterCmd),
    ChangeCharge(HChangeChargeCmd),
    ChangeSwEffect(HChangeSwEffectCmd),
    ChangeFwEffect(HChangeFwEffectCmd),
    ChangeProjEffect(HChangeProjEffectCmd),
}
impl HItemCommand {
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem, item_id: &rc::SsItemId) -> rc::Result<HCmdResp> {
        match self {
            Self::ChangeCharacter(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeSkill(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeImplant(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeBooster(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeShip(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeStructure(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeStance(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeSubsystem(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeModule(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeRig(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeDrone(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeFighter(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeCharge(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeSwEffect(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeFwEffect(cmd) => cmd.execute(core_ss, item_id),
            Self::ChangeProjEffect(cmd) => cmd.execute(core_ss, item_id),
        }
    }
}
