use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::val_options_br_resolve,
    val::{SolValInfo, ValInfoMode, ValOptions},
};

// Core commands
#[derive(Clone, Default)]
pub struct SolValCmd {
    options: ValOptions<ItemId>,
    fit_ids: Vec<FitId>,
    shared: SolValCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolValCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_ids: Vec<FitIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: SolValCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct SolValCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValInfoMode,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolValCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<ItemId>) -> Self {
        self.options = options;
        self
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit_ids.extend(fit_ids);
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValInfoMode) -> Self {
        self.shared.info_mode = info_mode;
        self
    }
}

impl SolValCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<ItemIdBr>) -> Self {
        self.options = options;
        self
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.fit_ids.extend(fit_ids);
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValInfoMode) -> Self {
        self.shared.info_mode = info_mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolValCmdBr {
    pub(in crate::val) fn br_resolve(self, resps: &CmdResps) -> Result<SolValCmd, BrResolveError> {
        Ok(SolValCmd {
            options: val_options_br_resolve(self.options, resps)?,
            fit_ids: resps.resolve_fit_ids(self.fit_ids)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolValCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> SolValInfo {
        let core_options = rc::val::ValOptionsSol {
            fit_ids: self.fit_ids,
            options: self.options,
        };
        match self.shared.info_mode {
            ValInfoMode::Simple => SolValInfo {
                passed: core_sol.validate_fast(&core_options),
                details: None,
            },
            ValInfoMode::Detailed => {
                let details = core_sol.validate_verbose(&core_options);
                SolValInfo {
                    passed: details.all_passed(),
                    details: Some(details),
                }
            }
        }
    }
}
