use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    info::{FitInfo, FitInfoMode, ItemInfoMode},
    shared::{OvrdCompact, OvrdMapLight},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdMapLight<ItemId, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitInfoCmdShared,
}
const impl Default for FitInfoCmd {
    fn default() -> Self {
        Self {
            item_mode: Default::default(),
            shared: Default::default(),
        }
    }
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdCompact<ItemIdBr, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitInfoCmdShared,
}
const impl Default for FitInfoCmdBr {
    fn default() -> Self {
        Self {
            item_mode: Default::default(),
            shared: Default::default(),
        }
    }
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
struct FitInfoCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: FitInfoMode,
}
const impl Default for FitInfoCmdShared {
    fn default() -> Self {
        Self {
            fit_mode: Default::default(),
        }
    }
}

// Extra context commands
#[derive(Clone)]
pub struct FitInfoCmdCtxFit {
    fit_id: FitId,
    core: FitInfoCmd,
}
#[doc(hidden)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitInfoCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitInfoCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmd {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.shared.fit_mode = mode;
        self
    }
    pub const fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl IntoIterator<Item = ItemId>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

impl FitInfoCmdBr {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.shared.fit_mode = mode;
        self
    }
    pub const fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl IntoIterator<Item = ItemIdBr>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmdBr {
    pub(in crate::info) const fn into_ctx_fit_br(self, fit_id: impl [const] Into<FitIdBr>) -> FitInfoCmdCtxFitBr {
        FitInfoCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> FitInfoCmd {
        FitInfoCmd {
            item_mode: OvrdMapLight::from_compact_with_br_resolution(self.item_mode, resps),
            shared: self.shared,
        }
    }
}

impl FitInfoCmdCtxFitBr {
    pub(in crate::info) fn br_resolve(self, resps: &CmdResps) -> Result<FitInfoCmdCtxFit, BrResolveError> {
        Ok(FitInfoCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::Fit) -> FitInfo {
        FitInfo::from_core(
            core_fit,
            &OvrdMapLight::from_default(self.shared.fit_mode),
            &self.item_mode,
        )
    }
}

impl FitInfoCmdCtxFit {
    pub(in crate::info) fn execute(self, core_sol: &mut rc::Sol) -> Result<FitInfo, FitGetFitInfoError> {
        let mut core_fit = core_sol.get_fit(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitInfoError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
