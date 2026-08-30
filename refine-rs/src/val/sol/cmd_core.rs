use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    shared::CmdResidue,
    val::{SolValResult, ValOptions, ValResultMode},
};

// Core commands
pub type SolValCmd = SolValCmdGen<FitId, ItemId>;
pub type SolValCmdBr = SolValCmdGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct SolValCmdGen<F, I> {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_ids: Vec<F>,
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValResultMode,
}
impl<F, I> Default for SolValCmdGen<F, I> {
    fn default() -> Self {
        Self {
            options: Default::default(),
            fit_ids: Default::default(),
            info_mode: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> SolValCmdGen<F, I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<I>) -> Self {
        self.options = options;
        self
    }
    pub fn with_fit_ids(mut self, fit_ids: impl IntoIterator<Item = F>) -> Self {
        self.fit_ids.extend(fit_ids);
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValResultMode) -> Self {
        self.info_mode = info_mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolValCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> SolValCmd {
        SolValCmd {
            options: self
                .options
                .filter_map_item_ids(|item_id_br| resps.resolve_item_id(item_id_br).ok()),
            fit_ids: resps.resolve_fit_ids_lossy(self.fit_ids),
            info_mode: self.info_mode,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> SolValCmdGen<F, I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}

impl SolValCmd {
    pub(crate) fn execute_owned(mut self, core_sol: &mut rc::SolarSystem) -> SolValResult {
        self.execute_borrowed(core_sol)
    }
    pub(crate) fn execute_borrowed(&mut self, core_sol: &mut rc::SolarSystem) -> SolValResult {
        let mut core_options = rc::val::ValOptionsSol {
            fit_ids: Vec::default(),
            options: ValOptions::default(),
        };
        // Avoid allocations by temporarily moving data to core options struct
        std::mem::swap(&mut self.fit_ids, &mut core_options.fit_ids);
        std::mem::swap(&mut self.options, &mut core_options.options);
        let result = match self.info_mode {
            ValResultMode::Simple => SolValResult {
                passed: core_sol.validate_fast(&core_options),
                details: None,
            },
            ValResultMode::Detailed => {
                let details = core_sol.validate_verbose(&core_options);
                SolValResult {
                    passed: details.all_passed(),
                    details: Some(details),
                }
            }
        };
        std::mem::swap(&mut self.fit_ids, &mut core_options.fit_ids);
        std::mem::swap(&mut self.options, &mut core_options.options);
        result
    }
}
