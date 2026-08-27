use crate::{
    CmdResps, FitId, FitIdBr, FleetId, FleetIdBr, ItemId, ItemIdBr,
    shared::{CmdResidue, OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStatsOptionsGen, FleetStatsOptionsGen, ItemStatsOptionsGen, SolStatsResp,
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
pub type SolStatsCmd = SolStatsCmdGen<FleetId, FitId, ItemId>;
pub type SolStatsCmdBr = SolStatsCmdGen<FleetIdBr, FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "L: serde::Deserialize<'de>, F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct SolStatsCmdGen<L, F, I> {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_options: OvrdCompact<L, FleetStatsOptionsGen<I>>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: OvrdCompact<F, FitStatsOptionsGen<F, I>>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<I, ItemStatsOptionsGen<F, I>>,
}
impl<L, F, I> Default for SolStatsCmdGen<L, F, I> {
    fn default() -> Self {
        Self {
            fleet_options: Default::default(),
            fit_options: Default::default(),
            item_options: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<L, F, I> SolStatsCmdGen<L, F, I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_default(mut self, options: FleetStatsOptionsGen<I>) -> Self {
        self.fleet_options.set_default(options);
        self
    }
    pub fn with_fleet_overrides(
        mut self,
        options: FleetStatsOptionsGen<I>,
        fleet_ids: impl Iterator<Item = L>,
    ) -> Self {
        self.fleet_options.add_overrides(options, fleet_ids);
        self
    }
    pub fn with_fit_default(mut self, options: FitStatsOptionsGen<F, I>) -> Self {
        self.fit_options.set_default(options);
        self
    }
    pub fn with_fit_overrides(mut self, options: FitStatsOptionsGen<F, I>, fit_ids: impl Iterator<Item = F>) -> Self {
        self.fit_options.add_overrides(options, fit_ids);
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
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolStatsCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> SolStatsCmd {
        SolStatsCmd {
            fleet_options: self.fleet_options.br_resolve(resps),
            fit_options: self.fit_options.br_resolve(resps),
            item_options: self.item_options.br_resolve(resps),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<L, F, I> SolStatsCmdGen<L, F, I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutInfallible
    }
}

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
