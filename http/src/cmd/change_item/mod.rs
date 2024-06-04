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
mod subsystem;
mod sw_effect;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeItemCommand {
    Character(HChangeCharacterCmd),
    Skill(HChangeSkillCmd),
    Implant(HChangeImplantCmd),
    Booster(HChangeBoosterCmd),
    Ship(HChangeShipCmd),
    Stance(HChangeStanceCmd),
    Subsystem(HChangeSubsystemCmd),
    Module(HChangeModuleCmd),
    Rig(HChangeRigCmd),
    Drone(HChangeDroneCmd),
    Fighter(HChangeFighterCmd),
    Charge(HChangeChargeCmd),
    SwEffect(HChangeSwEffectCmd),
    FwEffect(HChangeFwEffectCmd),
    ProjEffect(HChangeProjEffectCmd),
}
impl HChangeItemCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, item_id: &rc::SolItemId) -> rc::Result<HCmdResp> {
        match self {
            Self::Character(cmd) => cmd.execute(core_sol, item_id),
            Self::Skill(cmd) => cmd.execute(core_sol, item_id),
            Self::Implant(cmd) => cmd.execute(core_sol, item_id),
            Self::Booster(cmd) => cmd.execute(core_sol, item_id),
            Self::Ship(cmd) => cmd.execute(core_sol, item_id),
            Self::Stance(cmd) => cmd.execute(core_sol, item_id),
            Self::Subsystem(cmd) => cmd.execute(core_sol, item_id),
            Self::Module(cmd) => cmd.execute(core_sol, item_id),
            Self::Rig(cmd) => cmd.execute(core_sol, item_id),
            Self::Drone(cmd) => cmd.execute(core_sol, item_id),
            Self::Fighter(cmd) => cmd.execute(core_sol, item_id),
            Self::Charge(cmd) => cmd.execute(core_sol, item_id),
            Self::SwEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::FwEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::ProjEffect(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
