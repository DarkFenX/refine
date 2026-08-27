use crate::{
    CmdResp, CmdResps, ItemId, ItemIdBr,
    shared::CmdResidue,
    trial::{FitTryItemsCmdBr, FitTryItemsCmdGen},
};

pub(crate) type FitTryItemsEnumCmd = FitTryItemsEnumCmdGen<ItemId>;
pub type FitTryItemsEnumCmdBr = FitTryItemsEnumCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case"),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub enum FitTryItemsEnumCmdGen<I> {
    FitTryItems(FitTryItemsCmdGen<I>),
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
impl<I> FitTryItemsEnumCmdGen<I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self {
            Self::FitTryItems(cmd) => cmd.exec_residue(),
        }
    }
}

impl FitTryItemsEnumCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> CmdResp {
        match self {
            Self::FitTryItems(cmd) => CmdResp::TryItems(cmd.execute(core_fit)),
        }
    }
}
