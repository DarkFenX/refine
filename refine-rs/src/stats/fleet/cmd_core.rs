use rc::{ItemCommon, Lender};

use crate::{
    CmdResps, FitId, FitIdBr, FleetId, FleetIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStats, FitStatsOptions, FitStatsOptionsBr, FleetStatsOptions, FleetStatsOptionsBr, FleetStatsResp,
        ItemStats, ItemStatsOptions, ItemStatsOptionsBr,
        exec_shared::{extend_fit_item_stats, get_ovrd_item_stats},
        fit::FitStatsOptionsResolved,
        item::ItemStatsOptionsResolved,
    },
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_options: FleetStatsOptions,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: OvrdCompact<FitId, FitStatsOptions>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<ItemId, ItemStatsOptions>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetStatsCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_options: FleetStatsOptionsBr,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: OvrdCompact<FitIdBr, FitStatsOptionsBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<ItemIdBr, ItemStatsOptionsBr>,
}

// Extra context commands
#[derive(Clone)]
pub struct FleetStatsCmdCtxFleet {
    fleet_id: FleetId,
    core: FleetStatsCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FleetStatsCmdCtxFleetBr {
    fleet_id: FleetIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetStatsCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet(mut self, options: FleetStatsOptions) -> Self {
        self.fleet_options = options;
        self
    }
    pub fn with_fit_default(mut self, options: FitStatsOptions) -> Self {
        self.fit_options.set_default(options);
        self
    }
    pub fn with_fit_overrides(mut self, options: FitStatsOptions, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit_options.add_overrides(options, fit_ids);
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

impl FleetStatsCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet(mut self, options: FleetStatsOptionsBr) -> Self {
        self.fleet_options = options;
        self
    }
    pub fn with_fit_default(mut self, options: FitStatsOptionsBr) -> Self {
        self.fit_options.set_default(options);
        self
    }
    pub fn with_fit_overrides(mut self, options: FitStatsOptionsBr, fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.fit_options.add_overrides(options, fit_ids);
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
impl FleetStatsCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FleetStatsCmd, BrResolveError> {
        Ok(FleetStatsCmd {
            fleet_options: self.fleet_options.br_resolve(resps)?,
            fit_options: self.fit_options.br_resolve(resps)?,
            item_options: self.item_options.br_resolve(resps)?,
        })
    }
}

impl FleetStatsCmdCtxFleetBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<FleetStatsCmdCtxFleet, BrResolveError> {
        Ok(FleetStatsCmdCtxFleet {
            fleet_id: resps.resolve_fleet_id(self.fleet_id)?,
            core: self.core.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStatsResp {
        let fit_options: OvrdMapHeavy<_, FitStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.fit_options);
        let item_options: OvrdMapHeavy<_, ItemStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.item_options);
        let fits = match fit_options.get_default().is_any_stat_requested() {
            true => get_fleet_fit_stats(core_fleet, &fit_options),
            false => get_fleet_fit_stats_ovrd(core_fleet, &fit_options),
        };
        let items = match item_options.get_default().is_any_stat_requested() {
            true => get_fleet_item_stats(core_fleet, &item_options),
            false => get_fleet_item_stats_ovrd(core_fleet, &item_options),
        };
        FleetStatsResp {
            fleet: self.fleet_options.stat_resolve().execute(core_fleet),
            fits,
            items,
        }
    }
}
fn get_fleet_fit_stats(
    core_fleet: &mut rc::FleetMut,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
) -> Vec<(FitId, FitStats)> {
    core_fleet
        .iter_fits_mut()
        .map_into_iter(|mut core_fit| {
            let fit_id = core_fit.get_fit_id();
            let fit_stats = fit_options.get(&fit_id).execute(&mut core_fit);
            (fit_id, fit_stats)
        })
        .collect()
}
fn get_fleet_fit_stats_ovrd(
    core_fleet: &mut rc::FleetMut,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
) -> Vec<(FitId, FitStats)> {
    let fleet_id = core_fleet.get_fleet_id();
    let core_sol = core_fleet.get_sol_mut();
    let mut stats = Vec::with_capacity(fit_options.override_len());
    for (fit_id, options) in fit_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_fit) = core_sol.get_fit_mut(&fit_id) else {
            continue;
        };
        if core_fit.get_fleet().map(|core_fit_fleet| core_fit_fleet.get_fleet_id()) != Some(fleet_id) {
            continue;
        }
        stats.push((fit_id, options.execute(&mut core_fit)));
    }
    stats
}
fn get_fleet_item_stats(
    core_fleet: &mut rc::FleetMut,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
) -> Vec<(ItemId, ItemStats)> {
    let mut stats = Vec::new();
    let mut core_fits = core_fleet.iter_fits_mut();
    while let Some(mut core_fit) = core_fits.next() {
        extend_fit_item_stats(&mut core_fit, item_options, &mut stats);
    }
    stats
}
fn get_fleet_item_stats_ovrd(
    core_fleet: &mut rc::FleetMut,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
) -> Vec<(ItemId, ItemStats)> {
    let fleet_id = core_fleet.get_fleet_id();
    get_ovrd_item_stats(core_fleet.get_sol_mut(), item_options, |core_item| {
        core_item
            .get_fit()
            .and_then(|core_item_fit| core_item_fit.get_fleet().map(|fleet| fleet.get_fleet_id()))
            == Some(fleet_id)
    })
}

impl FleetStatsCmdCtxFleet {
    fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<FleetStatsResp, FleetGetFleetStatsError> {
        let mut core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.core.execute(&mut core_fleet))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetGetFleetStatsError {
    #[error(transparent)]
    FleetGet(#[from] rc::err::GetFleetError),
}
