use rc::{ItemCommon, Lender};

use crate::{
    ItemId,
    shared::OvrdMapHeavy,
    stats::{ItemStats, item::ItemStatsOptionsResolved},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Items
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn extend_fit_item_stats(
    core_fit: &mut rc::FitMut,
    item_options: &OvrdMapHeavy<ItemId, ItemStatsOptionsResolved>,
    stats: &mut Vec<(ItemId, ItemStats)>,
) {
    stats.extend(core_fit.iter_items_mut().map_into_iter(|mut core_item| {
        let item_id = core_item.get_item_id();
        let item_stats = item_options.get(&item_id).execute(&mut core_item);
        (item_id, item_stats)
    }));
}

pub(super) fn get_ovrd_item_stats<M>(
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
