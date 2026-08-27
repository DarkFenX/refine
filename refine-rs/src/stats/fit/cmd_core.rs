use rc::ItemCommon;

use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{BrResolveInfallible, CmdResidue, OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStatsOptions, FitStatsOptionsBr, FitStatsResp, ItemStatsOptions, ItemStatsOptionsBr,
        exec_shared::{extend_stats_for_passed_items, get_stats_for_items_in_overrides},
        item::ItemStatsOptionsResolved,
    },
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
impl FitStatsCmdBr {
    fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}
impl FitStatsCmdCtxFitBr {
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
