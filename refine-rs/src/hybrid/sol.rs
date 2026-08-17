use crate::{
    CmdResp, CmdResps, SolChangeEnumCmd, SolChangeEnumCmdBr,
    err::{BrResolveError, SolChangeEnumError},
};

pub(crate) enum SolHybridCmd {
    Ctl(SolChangeEnumCmd),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum SolHybridCmdBr {
    Ctl(SolChangeEnumCmdBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolHybridCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolHybridCmd, BrResolveError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => SolHybridCmd::Ctl(ctl_cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolHybridCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolHybridError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => ctl_cmd.execute(core_sol)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolHybridError {
    #[error(transparent)]
    Ctl(#[from] SolChangeEnumError),
}
