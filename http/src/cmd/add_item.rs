use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, change_sol},
    util::HExecError,
};

// Endpoint to add items provides no context just like solar system endpoint, so largely reuse
// commands from there
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HAddItemCommand {
    Booster(change_sol::HAddBoosterCmd),
    Character(change_sol::HSetCharacterCmd),
    Drone(change_sol::HAddDroneCmd),
    Fighter(change_sol::HAddFighterCmd),
    FwEffect(change_sol::HAddFwEffectCmd),
    Implant(change_sol::HAddImplantCmd),
    Module(change_sol::HAddModuleCmd),
    ProjEffect(change_sol::HAddProjEffectCmd),
    Rig(change_sol::HAddRigCmd),
    Service(change_sol::HAddServiceCmd),
    Ship(change_sol::HSetShipCmd),
    Skill(change_sol::HAddSkillCmd),
    Stance(change_sol::HSetStanceCmd),
    Subsystem(change_sol::HAddSubsystemCmd),
    SwEffect(change_sol::HAddSwEffectCmd),
}
impl HAddItemCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Booster(cmd) => cmd.execute(core_sol),
            Self::Character(cmd) => cmd.execute(core_sol),
            Self::Drone(cmd) => cmd.execute(core_sol),
            Self::Fighter(cmd) => cmd.execute(core_sol),
            Self::FwEffect(cmd) => cmd.execute(core_sol),
            Self::Implant(cmd) => cmd.execute(core_sol),
            Self::Module(cmd) => cmd.execute(core_sol),
            Self::ProjEffect(cmd) => Ok(cmd.execute(core_sol)),
            Self::Rig(cmd) => cmd.execute(core_sol),
            Self::Service(cmd) => cmd.execute(core_sol),
            Self::Ship(cmd) => cmd.execute(core_sol),
            Self::Skill(cmd) => cmd.execute(core_sol),
            Self::Stance(cmd) => cmd.execute(core_sol),
            Self::Subsystem(cmd) => cmd.execute(core_sol),
            Self::SwEffect(cmd) => Ok(cmd.execute(core_sol)),
        }
    }
}
