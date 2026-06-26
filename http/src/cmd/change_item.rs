use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{
            HBoosterChangeCmdICtx, HDroneChangeCmdICtxRIds, HFighterChangeCmdICtxRIds, HFwEffectChangeCmdICtx,
            HImplantChangeCmdICtx, HModuleChangeCmdICtxRIds, HRigChangeCmdICtx, HServiceChangeCmdICtx,
            HSkillChangeCmdICtx, HSubsystemChangeCmdICtx,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemChangeCmd {
    Booster(HBoosterChangeCmdICtx),
    Drone(HDroneChangeCmdICtxRIds),
    Fighter(HFighterChangeCmdICtxRIds),
    FwEffect(HFwEffectChangeCmdICtx),
    Implant(HImplantChangeCmdICtx),
    Module(HModuleChangeCmdICtxRIds),
    Rig(HRigChangeCmdICtx),
    Service(HServiceChangeCmdICtx),
    Skill(HSkillChangeCmdICtx),
    Subsystem(HSubsystemChangeCmdICtx),
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
            Self::Fighter(cmd) => cmd.execute(core_sol, item_id),
            Self::FwEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::Implant(cmd) => cmd.execute(core_sol, item_id),
            Self::Module(cmd) => cmd.execute(core_sol, item_id),
            Self::Rig(cmd) => cmd.execute(core_sol, item_id),
            Self::Service(cmd) => cmd.execute(core_sol, item_id),
            Self::Skill(cmd) => cmd.execute(core_sol, item_id),
            Self::Subsystem(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
