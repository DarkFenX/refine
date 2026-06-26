use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        old_change_item::{HChangeCharacterCmd, HChangeStanceCmd},
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeItemCommand {
    Character(HChangeCharacterCmd),
    Stance(HChangeStanceCmd),
}
impl HChangeItemCommand {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Character(cmd) => cmd.execute(core_sol, item_id),
            Self::Stance(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
