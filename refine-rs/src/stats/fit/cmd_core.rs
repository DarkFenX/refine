use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{OvrdCompact, OvrdMapHeavy},
    stats::{FitStatsOptions, FitStatsOptionsBr, FitStatsResp, ItemStatsOptions, ItemStatsOptionsBr},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: FitStatsOptions,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<ItemId, ItemStatsOptions>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitStatsCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: FitStatsOptionsBr,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<ItemIdBr, ItemStatsOptionsBr>,
}

// Extra context commands
#[derive(Clone)]
pub struct FitStatsCmdCtxFit {
    fit_id: FitId,
    core: FitStatsCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitStatsCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitStatsCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, options: FitStatsOptions) -> Self {
        self.fit_options = options;
        self
    }
    pub fn with_item_default(mut self, options: ItemStatsOptions) -> Self {
        self.item_options.set_default(options);
        self
    }
    pub fn with_item_overrides(mut self, options: ItemStatsOptions, item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.item_options.add_overrides(options, item_ids);
        self
    }
}

impl FitStatsCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, options: FitStatsOptionsBr) -> Self {
        self.fit_options = options;
        self
    }
    pub fn with_item_default(mut self, options: ItemStatsOptionsBr) -> Self {
        self.item_options.set_default(options);
        self
    }
    pub fn with_item_overrides(
        mut self,
        options: ItemStatsOptionsBr,
        item_ids: impl Iterator<Item = ItemIdBr>,
    ) -> Self {
        self.item_options.add_overrides(options, item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FitStatsCmd, BrResolveError> {
        Ok(FitStatsCmd {
            fit_options: self.fit_options.br_resolve(resps)?,
            item_options: self.item_options.br_resolve(resps)?,
        })
    }
}

impl FitStatsCmdCtxFitBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FitStatsCmdCtxFit, BrResolveError> {
        Ok(FitStatsCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStatsResp {
        let item_options = OvrdMapHeavy::from_compact(self.item_options);
        FitStatsResp {
            fit: self.fit_options.stat_resolve().execute(core_fit),
            items: Vec::new(),
        }
    }
}

impl FitStatsCmdCtxFit {
    fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<FitStatsResp, FitGetFitStatsError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitStatsError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}
