use crate::{
    CmdResps, FitId, FitIdBr, FleetId, FleetIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStatsOptions, FitStatsOptionsBr, FleetStatsOptions, FleetStatsOptionsBr, ItemStatsOptions,
        ItemStatsOptionsBr, SolStatsResp,
        exec_shared::{
            extend_stats_for_passed_items, get_stats_for_fits_in_overrides, get_stats_for_fleets_in_overrides,
            get_stats_for_items_in_overrides, get_stats_for_passed_fits, get_stats_for_passed_fleets,
        },
        fit::FitStatsOptionsResolved,
        fleet::FleetStatsOptionsResolved,
        item::ItemStatsOptionsResolved,
    },
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_options: OvrdCompact<FleetId, FleetStatsOptions>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: OvrdCompact<FitId, FitStatsOptions>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<ItemId, ItemStatsOptions>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolStatsCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_options: OvrdCompact<FleetIdBr, FleetStatsOptionsBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: OvrdCompact<FitIdBr, FitStatsOptionsBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<ItemIdBr, ItemStatsOptionsBr>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolStatsCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_default(mut self, options: FleetStatsOptions) -> Self {
        self.fleet_options.set_default(options);
        self
    }
    pub fn with_fleet_overrides(
        mut self,
        options: FleetStatsOptions,
        fleet_ids: impl Iterator<Item = FleetId>,
    ) -> Self {
        self.fleet_options.add_overrides(options, fleet_ids);
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

impl SolStatsCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_default(mut self, options: FleetStatsOptionsBr) -> Self {
        self.fleet_options.set_default(options);
        self
    }
    pub fn with_fleet_overrides(
        mut self,
        options: FleetStatsOptionsBr,
        fleet_ids: impl Iterator<Item = FleetIdBr>,
    ) -> Self {
        self.fleet_options.add_overrides(options, fleet_ids);
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
impl SolStatsCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolStatsCmd, BrResolveError> {
        Ok(SolStatsCmd {
            fleet_options: self.fleet_options.br_resolve(resps)?,
            fit_options: self.fit_options.br_resolve(resps)?,
            item_options: self.item_options.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolStatsCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> SolStatsResp {
        let fleet_options: OvrdMapHeavy<_, FleetStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.fleet_options);
        let fit_options: OvrdMapHeavy<_, FitStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.fit_options);
        let item_options: OvrdMapHeavy<_, ItemStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.item_options);
        // Everything in a solar system belongs to it, so overridden entities need no membership
        // check - unlike with fit and fleet commands
        let fleets = match fleet_options.get_default().is_any_stat_requested() {
            true => get_stats_for_passed_fleets(core_sol.iter_fleets_mut(), &fleet_options),
            false => get_stats_for_fleets_in_overrides(core_sol, &fleet_options),
        };
        let fits = match fit_options.get_default().is_any_stat_requested() {
            true => get_stats_for_passed_fits(core_sol.iter_fits_mut(), &fit_options),
            false => get_stats_for_fits_in_overrides(core_sol, &fit_options, |_| true),
        };
        let items = match item_options.get_default().is_any_stat_requested() {
            true => {
                let mut stats = Vec::new();
                extend_stats_for_passed_items(core_sol.iter_items_mut(), &item_options, &mut stats);
                stats
            }
            false => get_stats_for_items_in_overrides(core_sol, &item_options, |_| true),
        };
        SolStatsResp { fleets, fits, items }
    }
}
