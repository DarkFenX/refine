use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{
            HBoosterAddCmdFCtxRIds, HDroneAddCmdFCtxRIds, HFwEffectAddCmdFCtxRIds, HImplantAddCmdFCtxRIds,
            HRigAddCmdFCtxRIds, HServiceAddCmdFCtxRIds, HSkillAddCmdFCtxRIds, HSubsystemAddCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemAddCmd {
    Booster(HBoosterAddCmdFCtxRIds),
    Drone(HDroneAddCmdFCtxRIds),
    FwEffect(HFwEffectAddCmdFCtxRIds),
    Implant(HImplantAddCmdFCtxRIds),
    Rig(HRigAddCmdFCtxRIds),
    Service(HServiceAddCmdFCtxRIds),
    Skill(HSkillAddCmdFCtxRIds),
    Subsystem(HSubsystemAddCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Booster(cmd) => cmd.execute(core_sol),
            Self::Drone(cmd) => cmd.execute(core_sol),
            Self::FwEffect(cmd) => cmd.execute(core_sol),
            Self::Implant(cmd) => cmd.execute(core_sol),
            Self::Rig(cmd) => cmd.execute(core_sol),
            Self::Service(cmd) => cmd.execute(core_sol),
            Self::Skill(cmd) => cmd.execute(core_sol),
            Self::Subsystem(cmd) => cmd.execute(core_sol),
        }
    }
}
