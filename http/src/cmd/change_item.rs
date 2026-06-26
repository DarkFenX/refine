use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{
            HAutochargeChangeCmdICtx, HBoosterChangeCmdICtx, HChargeChangeCmdICtx, HDroneChangeCmdICtxRIds,
            HFighterChangeCmdICtxRIds, HFwEffectChangeCmdICtx, HImplantChangeCmdICtx, HModuleChangeCmdICtxRIds,
            HProjEffectChangeCmdICtxRIds, HRigChangeCmdICtx, HServiceChangeCmdICtx, HSkillChangeCmdICtx,
            HSubsystemChangeCmdICtx, HSwEffectChangeCmdICtx,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemChangeCmd {
    Autocharge(HAutochargeChangeCmdICtx),
    Booster(HBoosterChangeCmdICtx),
    Charge(HChargeChangeCmdICtx),
    Drone(HDroneChangeCmdICtxRIds),
    Fighter(HFighterChangeCmdICtxRIds),
    FwEffect(HFwEffectChangeCmdICtx),
    Implant(HImplantChangeCmdICtx),
    Module(HModuleChangeCmdICtxRIds),
    ProjEffect(HProjEffectChangeCmdICtxRIds),
    Rig(HRigChangeCmdICtx),
    Service(HServiceChangeCmdICtx),
    Skill(HSkillChangeCmdICtx),
    Subsystem(HSubsystemChangeCmdICtx),
    SwEffect(HSwEffectChangeCmdICtx),
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
            Self::Autocharge(cmd) => cmd.execute(core_sol, item_id),
            Self::Booster(cmd) => cmd.execute(core_sol, item_id),
            Self::Charge(cmd) => cmd.execute(core_sol, item_id),
            Self::Drone(cmd) => cmd.execute(core_sol, item_id),
            Self::Fighter(cmd) => cmd.execute(core_sol, item_id),
            Self::FwEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::Implant(cmd) => cmd.execute(core_sol, item_id),
            Self::Module(cmd) => cmd.execute(core_sol, item_id),
            Self::ProjEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::Rig(cmd) => cmd.execute(core_sol, item_id),
            Self::Service(cmd) => cmd.execute(core_sol, item_id),
            Self::Skill(cmd) => cmd.execute(core_sol, item_id),
            Self::Subsystem(cmd) => cmd.execute(core_sol, item_id),
            Self::SwEffect(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
