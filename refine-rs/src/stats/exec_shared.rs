use rc::{ExactSizeLender, ItemCommon, Lender};

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
    core_fleets: rc::EntityIter<'_, rc::Fleet<'_>>,
    fleet_options: &OvrdMapHeavy<FleetId, FleetStatsOptionsResolved>,
) -> Vec<(FleetId, FleetStats)> {
    core_fleets
        .map_into_iter(|mut core_fleet| {
            let fleet_id = core_fleet.get_fleet_id();
            let fleet_stats = fleet_options.get(&fleet_id).execute(&mut core_fleet.stats());
            (fleet_id, fleet_stats)
        })
        .collect()
}
pub(in crate::stats) fn get_stats_for_fleets_in_overrides(
    core_sol: &mut rc::Sol,
    fleet_options: &OvrdMapHeavy<FleetId, FleetStatsOptionsResolved>,
) -> Vec<(FleetId, FleetStats)> {
    let mut stats = Vec::with_capacity(fleet_options.override_len());
    for (fleet_id, options) in fleet_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_fleet) = core_sol.get_fleet(&fleet_id) else {
            continue;
        };
        stats.push((fleet_id, options.execute(&mut core_fleet.stats())));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fits
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn get_stats_for_passed_fits(
    core_fits: rc::EntityIter<'_, rc::Fit<'_>>,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
) -> Vec<(FitId, FitStats)> {
    core_fits
        .map_into_iter(|mut core_fit| {
            let fit_id = core_fit.get_fit_id();
            let fit_stats = fit_options.get(&fit_id).execute(&mut core_fit.stats());
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
        let Some(mut core_fit) = core_fit_src.get_core_fit(&fit_id) else {
            continue;
        };
        stats.push((fit_id, options.execute(&mut core_fit.stats())));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Items
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn extend_stats_for_passed_items(
    core_items: rc::EntityIter<'_, rc::Item<'_>>,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    stats: &mut Vec<(ItemId, ItemStats)>,
) {
    // Passed items do not include sub-items, so get stats for sub-items of every passed item
    stats.reserve(core_items.len());
    core_items.for_each(|mut core_item| {
        push_item_stats(&mut core_item, item_options, stats);
        core_item
            .iter_subitems()
            .for_each(|mut core_sub_item| push_item_stats(&mut core_sub_item, item_options, stats));
    });
}

fn push_item_stats(
    core_item: &mut rc::Item,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    stats: &mut Vec<(ItemId, ItemStats)>,
) {
    let item_id = core_item.get_item_id();
    let item_stats = item_options.get(&item_id).execute(&mut core_item.stats());
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
        let Some(mut core_item) = core_item_src.get_core_item(&item_id) else {
            continue;
        };
        stats.push((item_id, options.execute(&mut core_item.stats())));
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
    fn get_core_fit(&mut self, fit_id: &FitId) -> Option<rc::Fit<'_>>;
}
impl FitSource for rc::Sol {
    fn get_core_fit(&mut self, fit_id: &FitId) -> Option<rc::Fit<'_>> {
        self.get_fit(fit_id).ok()
    }
}
impl FitSource for rc::Fleet<'_> {
    fn get_core_fit(&mut self, fit_id: &FitId) -> Option<rc::Fit<'_>> {
        self.get_fit(fit_id).ok()
    }
}

pub(in crate::stats) trait ItemSource {
    fn get_core_item(&mut self, item_id: &ItemId) -> Option<rc::Item<'_>>;
}
impl ItemSource for rc::Sol {
    fn get_core_item(&mut self, item_id: &ItemId) -> Option<rc::Item<'_>> {
        self.get_item(item_id).ok()
    }
}
impl ItemSource for rc::Fit<'_> {
    fn get_core_item(&mut self, item_id: &ItemId) -> Option<rc::Item<'_>> {
        self.get_item(item_id).ok()
    }
}
impl ItemSource for rc::Fleet<'_> {
    fn get_core_item(&mut self, item_id: &ItemId) -> Option<rc::Item<'_>> {
        self.get_item(item_id).ok()
    }
}
