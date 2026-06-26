use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{HBoosterChangeCmdICtx, HDroneChangeCmdICtxRIds, HImplantChangeCmdICtx, HRigChangeCmdICtx},
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemChangeCmd {
    Booster(HBoosterChangeCmdICtx),
    Drone(HDroneChangeCmdICtxRIds),
    Implant(HImplantChangeCmdICtx),
    Rig(HRigChangeCmdICtx),
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
            Self::Booster(cmd) => cmd.execute(core_sol, item_id),
            Self::Drone(cmd) => cmd.execute(core_sol, item_id),
            Self::Implant(cmd) => cmd.execute(core_sol, item_id),
            Self::Rig(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
