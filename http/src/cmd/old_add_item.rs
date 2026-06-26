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
    Ship(old_change_sol::HSetShipCmd),
    Stance(old_change_sol::HSetStanceCmd),
}
impl HAddItemCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Character(cmd) => cmd.execute(core_sol),
            Self::Ship(cmd) => cmd.execute(core_sol),
            Self::Stance(cmd) => cmd.execute(core_sol),
        }
    }
}
