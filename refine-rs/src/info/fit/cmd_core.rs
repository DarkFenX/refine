use crate::{
    CmdResps, FitId, FitIdBr, FitInfo, FitInfoMode, ItemId, ItemIdBr, ItemInfoMode,
    err::BrResolveError,
    shared::{CmdResidue, OvrdCompact, OvrdMapLight},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdMapLight<ItemId, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    item_mode: OvrdCompact<ItemIdBr, ItemInfoMode>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
struct FitInfoCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_mode: FitInfoMode,
}

// Extra context commands
#[derive(Clone)]
pub struct FitInfoCmdCtxFit {
    fit_id: FitId,
    core: FitInfoCmd,
}
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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.shared.fit_mode = mode;
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

impl FitInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.shared.fit_mode = mode;
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode.set_default(mode);
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item_mode.add_overrides(mode, item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmdBr {
    pub(in crate::info) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> FitInfoCmdCtxFitBr {
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
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl FitInfoCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl FitInfoCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutFallible
    }
}

impl FitInfoCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitInfo {
        FitInfo::from_core(
            core_fit,
            &OvrdMapLight::from_default(self.shared.fit_mode),
            &self.item_mode,
        )
    }
}

impl FitInfoCmdCtxFit {
    pub(in crate::info) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<FitInfo, FitGetFitInfoError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitInfoError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
