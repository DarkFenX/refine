use crate::{
    CmdResp, CmdResps,
    trial::{FitTryItemsCmd, FitTryItemsCmdBr},
};

#[derive(Clone)]
pub(crate) enum FitTryItemsEnumCmd {
    FitTryItems(FitTryItemsCmd),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum FitTryItemsEnumCmdBr {
    FitTryItems(FitTryItemsCmdBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl FitTryItemsCmdBr {
    pub(crate) fn into_fit_try_br(self) -> FitTryItemsEnumCmdBr {
        FitTryItemsEnumCmdBr::FitTryItems(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> FitTryItemsEnumCmd {
        match self {
            Self::FitTryItems(cmd) => FitTryItemsEnumCmd::FitTryItems(cmd.br_resolve(resps)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitTryItemsEnumCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> CmdResp {
        match self {
            Self::FitTryItems(cmd) => CmdResp::TryItems(cmd.execute(core_fit)),
        }
    }
}
