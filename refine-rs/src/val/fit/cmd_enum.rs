use crate::{
    CmdResp, CmdResps, ItemId, ItemIdBr,
    shared::CmdResidue,
    val::{FitValCmdBr, FitValCmdGen},
};

pub(crate) type FitValEnumCmd = FitValEnumCmdGen<ItemId>;
pub type FitValEnumCmdBr = FitValEnumCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case"),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub enum FitValEnumCmdGen<I> {
    FitValidate(FitValCmdGen<I>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl FitValCmdBr {
    pub(crate) fn into_fit_val_br(self) -> FitValEnumCmdBr {
        FitValEnumCmdBr::FitValidate(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> FitValEnumCmd {
        match self {
            Self::FitValidate(cmd) => FitValEnumCmd::FitValidate(cmd.br_resolve(resps)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValEnumCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self {
            Self::FitValidate(cmd) => cmd.exec_residue(),
        }
    }
}

impl FitValEnumCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> CmdResp {
        match self {
            Self::FitValidate(cmd) => cmd.execute(core_fit).into(),
        }
    }
}
