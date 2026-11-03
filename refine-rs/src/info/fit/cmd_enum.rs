use crate::{
    CmdResp, CmdResps, FitInfoCmd, FitInfoCmdBr, ItemIdBr, ItemInfoCmdBr,
    err::{BrResolveError, FitItemGetItemInfoError},
    info::item::{ItemInfoCmdCtxItem, ItemInfoCmdCtxItemBr},
};

#[derive(Clone)]
pub(crate) enum FitInfoEnumCmd {
    FitInfo(FitInfoCmd),
    ItemInfo(ItemInfoCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum FitInfoEnumCmdBr {
    FitInfo(FitInfoCmdBr),
    ItemInfo(ItemInfoCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Not public because there is no direct consumer of the command; this command is usable only via
// hybrid batching endpoint
impl FitInfoCmdBr {
    pub(crate) fn into_fit_info_br(self) -> FitInfoEnumCmdBr {
        FitInfoEnumCmdBr::FitInfo(self)
    }
}
impl ItemInfoCmdBr {
    pub(crate) fn into_fit_info_br(self, item_id: impl Into<ItemIdBr>) -> FitInfoEnumCmdBr {
        FitInfoEnumCmdBr::ItemInfo(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<FitInfoEnumCmd, BrResolveError> {
        Ok(match self {
            Self::FitInfo(cmd) => FitInfoEnumCmd::FitInfo(cmd.br_resolve(resps)),
            Self::ItemInfo(cmd) => FitInfoEnumCmd::ItemInfo(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoEnumCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::Fit) -> Result<CmdResp, FitInfoEnumError> {
        Ok(match self {
            Self::FitInfo(cmd) => cmd.execute(core_fit).into(),
            Self::ItemInfo(cmd) => cmd.execute_via_fit(core_fit)?.into(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitInfoEnumError {
    #[error(transparent)]
    Item(#[from] FitItemGetItemInfoError),
}
