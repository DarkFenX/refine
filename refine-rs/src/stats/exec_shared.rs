use rc::{ItemCommon, Lender};

use crate::{
    FitId, FleetId, ItemId,
    shared::OvrdMapHeavy,
    stats::{
        FitStats, FleetStats, ItemStats, fit::FitStatsOptionsResolved, fleet::FleetStatsOptionsResolved,
        item::ItemStatsOptionsResolved,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fleets
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn get_stats_for_passed_fleets(
    core_fleets: rc::MutIter<'_, rc::FleetMut<'_>>,
    fleet_options: &OvrdMapHeavy<FleetId, FleetStatsOptionsResolved>,
) -> Vec<(FleetId, FleetStats)> {
    core_fleets
        .map_into_iter(|mut core_fleet| {
            let fleet_id = core_fleet.get_fleet_id();
            let fleet_stats = fleet_options.get(&fleet_id).execute(&mut core_fleet);
            (fleet_id, fleet_stats)
        })
        .collect()
}
pub(in crate::stats) fn get_stats_for_fleets_in_overrides(
    core_sol: &mut rc::SolarSystem,
    fleet_options: &OvrdMapHeavy<FleetId, FleetStatsOptionsResolved>,
) -> Vec<(FleetId, FleetStats)> {
    let mut stats = Vec::with_capacity(fleet_options.override_len());
    for (fleet_id, options) in fleet_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_fleet) = core_sol.get_fleet_mut(&fleet_id) else {
            continue;
        };
        stats.push((fleet_id, options.execute(&mut core_fleet)));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fits
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn get_stats_for_passed_fits(
    core_fits: rc::MutIter<'_, rc::FitMut<'_>>,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
) -> Vec<(FitId, FitStats)> {
    core_fits
        .map_into_iter(|mut core_fit| {
            let fit_id = core_fit.get_fit_id();
            let fit_stats = fit_options.get(&fit_id).execute(&mut core_fit);
            (fit_id, fit_stats)
        })
        .collect()
}

pub(in crate::stats) fn get_stats_for_fits_in_overrides<M>(
    core_sol: &mut rc::SolarSystem,
    fit_options: &OvrdMapHeavy<FitId, FitStatsOptionsResolved>,
    is_member: M,
) -> Vec<(FitId, FitStats)>
where
    M: Fn(&rc::FitMut) -> bool,
{
    let mut stats = Vec::with_capacity(fit_options.override_len());
    for (fit_id, options) in fit_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_fit) = core_sol.get_fit_mut(&fit_id) else {
            continue;
        };
        if !is_member(&core_fit) {
            continue;
        }
        stats.push((fit_id, options.execute(&mut core_fit)));
    }
    stats
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Items
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) fn extend_stats_for_passed_items(
    core_items: rc::MutIter<'_, rc::ItemMut<'_>>,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    stats: &mut Vec<(ItemId, ItemStats)>,
) {
    stats.extend(core_items.map_into_iter(|mut core_item| {
        let item_id = core_item.get_item_id();
        let item_stats = item_options.get(&item_id).execute(&mut core_item);
        (item_id, item_stats)
    }));
}

pub(in crate::stats) fn get_stats_for_items_in_overrides<M>(
    core_sol: &mut rc::SolarSystem,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    is_member: M,
) -> Vec<(ItemId, ItemStats)>
where
    M: Fn(&rc::ItemMut) -> bool,
{
    let mut stats = Vec::with_capacity(item_options.override_len());
    for (item_id, options) in item_options.iter_overrides() {
        if !options.is_any_stat_requested() {
            continue;
        }
        let Ok(mut core_item) = core_sol.get_item_mut(&item_id) else {
            continue;
        };
        if !is_member(&core_item) {
            continue;
        }
        stats.push((item_id, options.execute(&mut core_item)));
    }
    stats
}
