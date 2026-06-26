use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, old_change_sol},
    util::HExecError,
};

// Endpoint to add items provides no context just like solar system endpoint, so largely reuse
// commands from there
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HAddItemCommand {
    Character(old_change_sol::HSetCharacterCmd),
    Fighter(old_change_sol::HAddFighterCmd),
    FwEffect(old_change_sol::HAddFwEffectCmd),
    Module(old_change_sol::HAddModuleCmd),
    ProjEffect(old_change_sol::HAddProjEffectCmd),
    Service(old_change_sol::HAddServiceCmd),
    Ship(old_change_sol::HSetShipCmd),
    Skill(old_change_sol::HAddSkillCmd),
    Stance(old_change_sol::HSetStanceCmd),
    Subsystem(old_change_sol::HAddSubsystemCmd),
    SwEffect(old_change_sol::HAddSwEffectCmd),
}
impl HAddItemCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Character(cmd) => cmd.execute(core_sol),
            Self::Fighter(cmd) => cmd.execute(core_sol),
            Self::FwEffect(cmd) => cmd.execute(core_sol),
            Self::Module(cmd) => cmd.execute(core_sol),
            Self::ProjEffect(cmd) => cmd.execute(core_sol),
            Self::Service(cmd) => cmd.execute(core_sol),
            Self::Ship(cmd) => cmd.execute(core_sol),
            Self::Skill(cmd) => cmd.execute(core_sol),
            Self::Stance(cmd) => cmd.execute(core_sol),
            Self::Subsystem(cmd) => cmd.execute(core_sol),
            Self::SwEffect(cmd) => Ok(cmd.execute(core_sol)),
        }
    }
}
