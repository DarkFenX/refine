use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::val_options_br_resolve,
    val::{FitValResult, ValInfoMode, ValOptions},
};

// Core commands
#[derive(Clone, Default)]
pub struct FitValCmd {
    options: ValOptions<ItemId>,
    shared: FitValCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitValCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitValCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct FitValCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValInfoMode,
}

// Extra context commands
#[derive(Clone)]
pub struct FitValCmdCtxFit {
    fit_id: FitId,
    core: FitValCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitValCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitValCmdBr,
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
    pub fn with_info_mode(mut self, info_mode: ValInfoMode) -> Self {
        self.shared.info_mode = info_mode;
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
    pub fn with_info_mode(mut self, info_mode: ValInfoMode) -> Self {
        self.shared.info_mode = info_mode;
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
    pub(in crate::val) fn br_resolve(self, resps: &CmdResps) -> Result<FitValCmd, BrResolveError> {
        Ok(FitValCmd {
            options: val_options_br_resolve(self.options, resps)?,
            shared: self.shared,
        })
    }
}

impl FitValCmdCtxFitBr {
    pub(in crate::val) fn br_resolve(self, resps: &CmdResps) -> Result<FitValCmdCtxFit, BrResolveError> {
        Ok(FitValCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitValResult {
        match self.shared.info_mode {
            ValInfoMode::Simple => FitValResult {
                passed: core_fit.validate_fast(&self.options),
                details: None,
            },
            ValInfoMode::Detailed => {
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
    FitGet(#[from] rc::err::GetFitError),
}
