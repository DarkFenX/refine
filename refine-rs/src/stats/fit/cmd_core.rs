use rc::{ItemCommon, Lender};

use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStatsOptions, FitStatsOptionsBr, FitStatsResp, ItemStats, ItemStatsOptions, ItemStatsOptionsBr,
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
        let item_options: OvrdMapHeavy<_, ItemStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.item_options);
        let items = match item_options.get_default().is_any_stat_requested() {
            true => get_fit_item_stats(core_fit, &item_options),
            false => get_fit_item_stats_ovrd(core_fit, &item_options),
        };
        FitStatsResp {
            fit: self.fit_options.stat_resolve().execute(core_fit),
            items,
        }
    }
}
fn get_fit_item_stats(
    core_fit: &mut rc::FitMut,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
) -> Vec<(ItemId, ItemStats)> {
    core_fit
        .iter_items_mut()
        .map_into_iter(|mut core_item| {
            let item_id = core_item.get_item_id();
            let item_stats = item_options.get(&item_id).execute(&mut core_item);
            (item_id, item_stats)
        })
        .collect()
}
fn get_fit_item_stats_ovrd(
    core_fit: &mut rc::FitMut,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
) -> Vec<(ItemId, ItemStats)> {
    let fit_id = core_fit.get_fit_id();
    let core_sol = core_fit.get_sol_mut();
    let mut stats = Vec::with_capacity(item_options.override_len());
    for (item_id, options) in item_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_item) = core_sol.get_item_mut(&item_id) else {
            continue;
        };
        if core_item.get_fit().map(|core_item_fit| core_item_fit.get_fit_id()) != Some(fit_id) {
            continue;
        }
        stats.push((item_id, options.execute(&mut core_item)));
    }
    stats
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
