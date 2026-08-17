use crate::{
    CmdResp, CmdResps, FitChangeEnumCmd, FitChangeEnumCmdBr,
    err::{BrResolveError, FitChangeEnumError},
};

pub(crate) enum FitHybridCmd {
    Ctl(FitChangeEnumCmd),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum FitHybridCmdBr {
    Ctl(FitChangeEnumCmdBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitHybridCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<FitHybridCmd, BrResolveError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => FitHybridCmd::Ctl(ctl_cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitHybridCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Result<CmdResp, FitHybridError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => ctl_cmd.execute(core_fit)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitHybridError {
    #[error(transparent)]
    Ctl(#[from] FitChangeEnumError),
}
