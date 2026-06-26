use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{
            HBoosterAddCmdFCtxRIds, HDroneAddCmdFCtxRIds, HFighterAddCmdFCtxRIds, HFwEffectAddCmdFCtxRIds,
            HImplantAddCmdFCtxRIds, HModuleAddCmdFCtxRIds, HProjEffectAddCmdFCtxRIds, HRigAddCmdFCtxRIds,
            HServiceAddCmdFCtxRIds, HSkillAddCmdFCtxRIds, HSubsystemAddCmdFCtxRIds, HSwEffectAddCmdFCtx,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemAddCmd {
    Booster(HBoosterAddCmdFCtxRIds),
    Drone(HDroneAddCmdFCtxRIds),
    Fighter(HFighterAddCmdFCtxRIds),
    FwEffect(HFwEffectAddCmdFCtxRIds),
    Implant(HImplantAddCmdFCtxRIds),
    Module(HModuleAddCmdFCtxRIds),
    ProjEffect(HProjEffectAddCmdFCtxRIds),
    Rig(HRigAddCmdFCtxRIds),
    Service(HServiceAddCmdFCtxRIds),
    Skill(HSkillAddCmdFCtxRIds),
    Subsystem(HSubsystemAddCmdFCtxRIds),
    SwEffect(HSwEffectAddCmdFCtx),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Booster(cmd) => cmd.execute(core_sol),
            Self::Drone(cmd) => cmd.execute(core_sol),
            Self::Fighter(cmd) => cmd.execute(core_sol),
            Self::FwEffect(cmd) => cmd.execute(core_sol),
            Self::Implant(cmd) => cmd.execute(core_sol),
            Self::Module(cmd) => cmd.execute(core_sol),
            Self::ProjEffect(cmd) => cmd.execute(core_sol),
            Self::Rig(cmd) => cmd.execute(core_sol),
            Self::Service(cmd) => cmd.execute(core_sol),
            Self::Skill(cmd) => cmd.execute(core_sol),
            Self::Subsystem(cmd) => cmd.execute(core_sol),
            Self::SwEffect(cmd) => Ok(cmd.execute(core_sol)),
        }
    }
}
