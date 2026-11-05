use rc::{ExactSizeLender, Lender};

use crate::{
    FitId, FleetId, ItemId,
    err::BrResolveError,
    shared::OvrdMapHeavy,
    stats::{
        FitStats, FleetStats, ItemStats, StatBrFallibleError, StatResult, fatal::StatErrorFatality,
        fit::FitStatsOptionsResolved, fleet::FleetStatsOptionsResolved, item::ItemStatsOptionsResolved,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fleets
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn get_stats_for_passed_fleets(
    core_fleets_stats: rc::EntityIter<'_, rc::stats::FleetStats<'_>>,
    fleet_options: &OvrdMapHeavy<FleetId, FleetStatsOptionsResolved>,
) -> Vec<(FleetId, FleetStats)> {
    core_fleets_stats
        .map_into_iter(|mut core_fleet_stats| {
            let fleet_id = core_fleet_stats.get_fleet_id();
            let fleet_stats = fleet_options.get(&fleet_id).execute(&mut core_fleet_stats);
            (fleet_id, fleet_stats)
        })
        .collect()
}
pub(in crate::stats) fn get_stats_for_fleets_in_overrides(
    core_sol_stats: &mut rc::stats::SolStats,
    fleet_options: &OvrdMapHeavy<FleetId, FleetStatsOptionsResolved>,
) -> Vec<(FleetId, FleetStats)> {
    let mut stats = Vec::with_capacity(fleet_options.override_len());
    for (fleet_id, options) in fleet_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_fleet_stats) = core_sol_stats.get_fleet(&fleet_id) else {
            continue;
        };
        stats.push((fleet_id, options.execute(&mut core_fleet_stats)));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fits
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn get_stats_for_passed_fits(
    core_fits_stats: rc::EntityIter<'_, rc::stats::FitStats<'_>>,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
) -> Vec<(FitId, FitStats)> {
    core_fits_stats
        .map_into_iter(|mut core_fit_stats| {
            let fit_id = core_fit_stats.get_fit_id();
            let fit_stats = fit_options.get(&fit_id).execute(&mut core_fit_stats);
            (fit_id, fit_stats)
        })
        .collect()
}

pub(in crate::stats) fn get_stats_for_fits_in_overrides<S>(
    core_fit_src: &mut S,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
) -> Vec<(FitId, FitStats)>
where
    S: FitSource + ?Sized,
{
    let mut stats = Vec::with_capacity(fit_options.override_len());
    for (fit_id, options) in fit_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Some(mut core_fit_stats) = core_fit_src.get_core_fit_stats(&fit_id) else {
            continue;
        };
        stats.push((fit_id, options.execute(&mut core_fit_stats)));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Items
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn extend_stats_for_passed_items(
    core_items_stats: rc::EntityIter<'_, rc::stats::ItemStats<'_>>,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    stats: &mut Vec<(ItemId, ItemStats)>,
) {
    // Passed items do not include sub-items, so get stats for sub-items of every passed item
    stats.reserve(core_items_stats.len());
    core_items_stats.for_each(|mut core_item_stats| {
        push_item_stats(&mut core_item_stats, item_options, stats);
        core_item_stats
            .iter_subitems()
            .for_each(|mut core_sub_item_stats| push_item_stats(&mut core_sub_item_stats, item_options, stats));
    });
}

fn push_item_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    stats: &mut Vec<(ItemId, ItemStats)>,
) {
    let item_id = core_item_stats.get_item_id();
    let item_stats = item_options.get(&item_id).execute(core_item_stats);
    stats.push((item_id, item_stats));
}

pub(in crate::stats) fn get_stats_for_items_in_overrides<S>(
    core_item_src: &mut S,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
) -> Vec<(ItemId, ItemStats)>
where
    S: ItemSource + ?Sized,
{
    let mut stats = Vec::with_capacity(item_options.override_len());
    for (item_id, options) in item_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Some(mut core_item_stats) = core_item_src.get_core_item_stats(&item_id) else {
            continue;
        };
        stats.push((item_id, options.execute(&mut core_item_stats)));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Per-option stat getters
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn collect_stats_err_inner<O, T, E, F>(options: &[O], mut getter: F) -> StatResult<T, E, !>
where
    F: FnMut(&O) -> Result<T, E>,
{
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match getter(option) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

pub(in crate::stats) fn collect_stats_err_outer_br<O, T, E, F>(
    options: &[Result<O, BrResolveError>],
    mut getter: F,
) -> StatResult<T, !, StatBrFallibleError<E>>
where
    F: FnMut(&O) -> Result<T, E>,
{
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option {
            Ok(option) => stats.push(getter(option).map_err(StatBrFallibleError::Stat)),
            Err(br_err) => stats.push(Err(StatBrFallibleError::BrResolve(br_err.clone()))),
        }
    }
    StatResult::Result(stats)
}

pub(in crate::stats) fn collect_stats_err_both_br<O, T, E, F>(
    options: &[Result<O, BrResolveError>],
    mut getter: F,
) -> StatResult<T, E, StatBrFallibleError<E>>
where
    E: StatErrorFatality,
    F: FnMut(&O) -> Result<T, E>,
{
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option {
            Ok(option) => match getter(option) {
                Ok(stat) => stats.push(Ok(stat)),
                Err(err) => match err.is_fatal() {
                    true => return StatResult::Error(err),
                    false => stats.push(Err(StatBrFallibleError::Stat(err))),
                },
            },
            Err(br_err) => stats.push(Err(StatBrFallibleError::BrResolve(br_err.clone()))),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter traits
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) trait FitSource {
    fn get_core_fit_stats(&mut self, fit_id: &FitId) -> Option<rc::stats::FitStats<'_>>;
}
impl FitSource for rc::stats::SolStats<'_> {
    fn get_core_fit_stats(&mut self, fit_id: &FitId) -> Option<rc::stats::FitStats<'_>> {
        self.get_fit(fit_id).ok()
    }
}
impl FitSource for rc::stats::FleetStats<'_> {
    fn get_core_fit_stats(&mut self, fit_id: &FitId) -> Option<rc::stats::FitStats<'_>> {
        self.get_fit(fit_id).ok()
    }
}

pub(in crate::stats) trait ItemSource {
    fn get_core_item_stats(&mut self, item_id: &ItemId) -> Option<rc::stats::ItemStats<'_>>;
}
impl ItemSource for rc::stats::SolStats<'_> {
    fn get_core_item_stats(&mut self, item_id: &ItemId) -> Option<rc::stats::ItemStats<'_>> {
        self.get_item(item_id).ok()
    }
}
impl ItemSource for rc::stats::FitStats<'_> {
    fn get_core_item_stats(&mut self, item_id: &ItemId) -> Option<rc::stats::ItemStats<'_>> {
        self.get_item(item_id).ok()
    }
}
impl ItemSource for rc::stats::FleetStats<'_> {
    fn get_core_item_stats(&mut self, item_id: &ItemId) -> Option<rc::stats::ItemStats<'_>> {
        self.get_item(item_id).ok()
    }
}
