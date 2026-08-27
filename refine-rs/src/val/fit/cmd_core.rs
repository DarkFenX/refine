use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::CmdResidue,
    val::{FitValResult, ValOptions, ValResultMode},
};

// Core commands
pub type FitValCmd = FitValCmdGen<ItemId>;
pub type FitValCmdBr = FitValCmdGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitValCmdGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValResultMode,
}
impl<I> Default for FitValCmdGen<I> {
    fn default() -> Self {
        Self {
            options: Default::default(),
            info_mode: Default::default(),
        }
    }
}

// Extra context commands
pub type FitValCmdCtxFit = FitValCmdCtxFitGen<FitId, ItemId>;
pub type FitValCmdCtxFitBr = FitValCmdCtxFitGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitValCmdCtxFitGen<F, I> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitValCmdGen<I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<ItemId>) -> Self {
        self.options = options;
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValResultMode) -> Self {
        self.info_mode = info_mode;
        self
    }
}

impl FitValCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<ItemIdBr>) -> Self {
        self.options = options;
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValResultMode) -> Self {
        self.info_mode = info_mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmdBr {
    pub(in crate::val) fn into_ctx_item_br(self, fit_id: impl Into<FitIdBr>) -> FitValCmdCtxFitBr {
        FitValCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmdBr {
    pub(in crate::val) fn br_resolve(self, resps: &CmdResps) -> FitValCmd {
        FitValCmd {
            options: self
                .options
                .filter_map_item_ids(|item_id_br| resps.resolve_item_id(item_id_br).ok()),
            info_mode: self.info_mode,
        }
    }
}

impl FitValCmdCtxFitBr {
    pub(in crate::val) fn br_resolve(self, resps: &CmdResps) -> Result<FitValCmdCtxFit, BrResolveError> {
        Ok(FitValCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmdBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl FitValCmdCtxFitBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutFallible
    }
}

impl FitValCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitValResult {
        match self.info_mode {
            ValResultMode::Simple => FitValResult {
                passed: core_fit.validate_fast(&self.options),
                details: None,
            },
            ValResultMode::Detailed => {
                let details = core_fit.validate_verbose(&self.options);
                FitValResult {
                    passed: details.all_passed(),
                    details: Some(details),
                }
            }
        }
    }
}

impl FitValCmdCtxFit {
    pub(in crate::val) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<FitValResult, FitGetFitValError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitValError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
