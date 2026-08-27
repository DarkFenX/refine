use rc::ItemCommon;

use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{BrResolveInfallible, CmdResidue, OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStatsOptionsGen, FitStatsResp, ItemStatsOptionsGen,
        exec_shared::{extend_stats_for_passed_items, get_stats_for_items_in_overrides},
        item::ItemStatsOptionsResolved,
    },
};

// Core commands
pub type FitStatsCmd = FitStatsCmdGen<FitId, ItemId>;
pub type FitStatsCmdBr = FitStatsCmdGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitStatsCmdGen<F, I> {
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: FitStatsOptionsGen<F, I>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<I, ItemStatsOptionsGen<F, I>>,
}
impl<F, I> Default for FitStatsCmdGen<F, I> {
    fn default() -> Self {
        Self {
            fit_options: Default::default(),
            item_options: Default::default(),
        }
    }
}

// Extra context commands
pub type FitStatsCmdCtxFit = FitStatsCmdCtxFitGen<FitId, ItemId>;
pub type FitStatsCmdCtxFitBr = FitStatsCmdCtxFitGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FitStatsCmdCtxFitGen<F, I> {
    fit_id: F,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitStatsCmdGen<F, I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FitStatsCmdGen<F, I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, options: FitStatsOptionsGen<F, I>) -> Self {
        self.fit_options = options;
        self
    }
    pub fn with_item_default(mut self, options: ItemStatsOptionsGen<F, I>) -> Self {
        self.item_options.set_default(options);
        self
    }
    pub fn with_item_overrides(
        mut self,
        options: ItemStatsOptionsGen<F, I>,
        item_ids: impl Iterator<Item = I>,
    ) -> Self {
        self.item_options.add_overrides(options, item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsCmdBr {
    pub(in crate::stats) fn into_ctx_fit_br(self, fit_id: impl Into<FitIdBr>) -> FitStatsCmdCtxFitBr {
        FitStatsCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsCmdBr {
    pub(super) fn br_resolve(self, resps: &CmdResps) -> FitStatsCmd {
        FitStatsCmd {
            fit_options: self.fit_options.br_resolve_infallible(resps),
            item_options: self.item_options.br_resolve(resps),
        }
    }
}

impl FitStatsCmdCtxFitBr {
    pub(in crate::stats) fn br_resolve(self, resps: &CmdResps) -> Result<FitStatsCmdCtxFit, BrResolveError> {
        Ok(FitStatsCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FitStatsCmdGen<F, I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl<F, I> FitStatsCmdCtxFitGen<F, I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutFallible
    }
}

impl FitStatsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStatsResp {
        let item_options: OvrdMapHeavy<_, ItemStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.item_options);
        let items = match item_options.get_default().is_any_stat_requested() {
            true => {
                let mut stats = Vec::new();
                extend_stats_for_passed_items(core_fit.iter_items_mut(), &item_options, &mut stats);
                stats
            }
            false => {
                let fit_id = core_fit.get_fit_id();
                get_stats_for_items_in_overrides(core_fit.get_sol_mut(), &item_options, |core_item| {
                    core_item.get_fit().map(|core_item_fit| core_item_fit.get_fit_id()) == Some(fit_id)
                })
            }
        };
        FitStatsResp {
            fit: self.fit_options.stat_resolve().execute(core_fit),
            items,
        }
    }
}
impl FitStatsCmdCtxFit {
    pub(in crate::stats) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<FitStatsResp, FitGetFitStatsError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute(&mut core_fit))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetFitStatsError {
    #[error(transparent)]
    FitGet(#[from] rc::err::FitGetError),
}
