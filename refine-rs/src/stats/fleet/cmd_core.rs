use rc::{ItemCommon, Lender};

use crate::{
    CmdResps, FitId, FitIdBr, FleetId, FleetIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::{BrResolveInfallible, CmdResidue, OvrdCompact, OvrdMapHeavy},
    stats::{
        FitStatsOptionsGen, FleetStatsOptionsGen, FleetStatsResp, ItemStatsOptionsGen,
        exec_shared::{
            extend_stats_for_passed_items, get_stats_for_fits_in_overrides, get_stats_for_items_in_overrides,
            get_stats_for_passed_fits,
        },
        fit::FitStatsOptionsResolved,
        item::ItemStatsOptionsResolved,
    },
};

// Core commands
pub type FleetStatsCmd = FleetStatsCmdGen<FitId, ItemId>;
pub type FleetStatsCmdBr = FleetStatsCmdGen<FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetStatsCmdGen<F, I> {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_options: FleetStatsOptionsGen<I>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_options: OvrdCompact<F, FitStatsOptionsGen<F, I>>,
    #[cfg_attr(feature = "serde", serde(default))]
    item_options: OvrdCompact<I, ItemStatsOptionsGen<F, I>>,
}
impl<F, I> Default for FleetStatsCmdGen<F, I> {
    fn default() -> Self {
        Self {
            fleet_options: Default::default(),
            fit_options: Default::default(),
            item_options: Default::default(),
        }
    }
}

// Extra context commands
pub type FleetStatsCmdCtxFleet = FleetStatsCmdCtxFleetGen<FleetId, FitId, ItemId>;
pub type FleetStatsCmdCtxFleetBr = FleetStatsCmdCtxFleetGen<FleetIdBr, FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "L: serde::Deserialize<'de>, F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetStatsCmdCtxFleetGen<L, F, I> {
    fleet_id: L,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetStatsCmdGen<F, I>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FleetStatsCmdGen<F, I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet(mut self, options: FleetStatsOptionsGen<I>) -> Self {
        self.fleet_options = options;
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsCmdBr {
    pub(in crate::stats) fn into_ctx_fleet_br(self, fleet_id: impl Into<FleetIdBr>) -> FleetStatsCmdCtxFleetBr {
        FleetStatsCmdCtxFleetBr {
            fleet_id: fleet_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsCmdBr {
    fn br_resolve(self, resps: &CmdResps) -> FleetStatsCmd {
        FleetStatsCmd {
            fleet_options: self.fleet_options.br_resolve_infallible(resps),
            fit_options: self.fit_options.br_resolve(resps),
            item_options: self.item_options.br_resolve(resps),
        }
    }
}

impl FleetStatsCmdCtxFleetBr {
    pub(in crate::stats) fn br_resolve(self, resps: &CmdResps) -> Result<FleetStatsCmdCtxFleet, BrResolveError> {
        Ok(FleetStatsCmdCtxFleet {
            fleet_id: resps.resolve_fleet_id(self.fleet_id)?,
            core: self.core.br_resolve(resps),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<L, F, I> FleetStatsCmdCtxFleetGen<L, F, I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::ImmutFallible
    }
}

impl FleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStatsResp {
        let fit_options: OvrdMapHeavy<_, FitStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.fit_options);
        let item_options: OvrdMapHeavy<_, ItemStatsOptionsResolved> =
            OvrdMapHeavy::from_compact_with_conversion(self.item_options);
        let fits = match fit_options.get_default().is_any_stat_requested() {
            true => get_stats_for_passed_fits(core_fleet.iter_fits_mut(), &fit_options),
            false => {
                let fleet_id = core_fleet.get_fleet_id();
                get_stats_for_fits_in_overrides(core_fleet.get_sol_mut(), &fit_options, |core_fit| {
                    core_fit.get_fleet().map(|core_fit_fleet| core_fit_fleet.get_fleet_id()) == Some(fleet_id)
                })
            }
        };
        let items = match item_options.get_default().is_any_stat_requested() {
            true => {
                let mut stats = Vec::new();
                let mut core_fits = core_fleet.iter_fits_mut();
                while let Some(mut core_fit) = core_fits.next() {
                    extend_stats_for_passed_items(core_fit.iter_items_mut(), &item_options, &mut stats);
                }
                stats
            }
            false => {
                let fleet_id = core_fleet.get_fleet_id();
                get_stats_for_items_in_overrides(core_fleet.get_sol_mut(), &item_options, |core_item| {
                    core_item
                        .get_fit()
                        .and_then(|core_item_fit| core_item_fit.get_fleet().map(|fleet| fleet.get_fleet_id()))
                        == Some(fleet_id)
                })
            }
        };
        FleetStatsResp {
            fleet: self.fleet_options.stat_resolve().execute(core_fleet),
            fits,
            items,
        }
    }
}

impl FleetStatsCmdCtxFleet {
    pub(in crate::stats) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<FleetStatsResp, FleetGetFleetStatsError> {
        let mut core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.core.execute(&mut core_fleet))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetGetFleetStatsError {
    #[error(transparent)]
    FleetGet(#[from] rc::err::FleetGetError),
}
