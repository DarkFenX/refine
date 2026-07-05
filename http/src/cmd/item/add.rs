use serde::Deserialize;

use crate::{
    cmd::{
        basic::{
            HBoosterAddCmdFCtxRIds, HCharacterSetCmdFCtxRIds, HDroneAddCmdFCtxRIds, HFighterAddCmdFCtxRIds,
            HFwEffectAddCmdFCtxRIds, HImplantAddCmdFCtxRIds, HModuleAddCmdFCtxRIds, HProjEffectAddCmdFCtxRIds,
            HRigAddCmdFCtxRIds, HServiceAddCmdFCtxRIds, HShipSetCmdFCtxRIds, HSkillAddCmdFCtxRIds,
            HStanceSetCmdFCtxRIds, HSubsystemAddCmdFCtxRIds, HSwEffectAddCmdFCtx,
        },
        shared::HCreatedItemIdsResp,
    },
    err::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemAddCmd {
    Booster(HBoosterAddCmdFCtxRIds),
    Character(HCharacterSetCmdFCtxRIds),
    Drone(HDroneAddCmdFCtxRIds),
    Fighter(HFighterAddCmdFCtxRIds),
    FwEffect(HFwEffectAddCmdFCtxRIds),
    Implant(HImplantAddCmdFCtxRIds),
    Module(HModuleAddCmdFCtxRIds),
    ProjEffect(HProjEffectAddCmdFCtxRIds),
    Rig(HRigAddCmdFCtxRIds),
    Service(HServiceAddCmdFCtxRIds),
    Ship(HShipSetCmdFCtxRIds),
    Skill(HSkillAddCmdFCtxRIds),
    Stance(HStanceSetCmdFCtxRIds),
    Subsystem(HSubsystemAddCmdFCtxRIds),
    SwEffect(HSwEffectAddCmdFCtx),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        match self {
            Self::Booster(cmd) => cmd.execute(core_sol),
            Self::Character(cmd) => cmd.execute(core_sol),
            Self::Drone(cmd) => cmd.execute(core_sol),
            Self::Fighter(cmd) => cmd.execute(core_sol),
            Self::FwEffect(cmd) => cmd.execute(core_sol),
            Self::Implant(cmd) => cmd.execute(core_sol),
            Self::Module(cmd) => cmd.execute(core_sol),
            Self::ProjEffect(cmd) => cmd.execute(core_sol),
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
