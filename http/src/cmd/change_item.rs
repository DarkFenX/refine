use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{HDroneChangeCmdICtxRIds, HImplantChangeCmdICtx},
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemChangeCmd {
    Drone(HDroneChangeCmdICtxRIds),
    Implant(HImplantChangeCmdICtx),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemChangeCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Drone(cmd) => cmd.execute(core_sol, item_id),
            Self::Implant(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
