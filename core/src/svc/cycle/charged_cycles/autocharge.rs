use crate::{rd::REffectKey, svc::cycle::charged_cycles::ChargedCycleCount, ud::UItem, util::InfCount};

pub(in crate::svc::cycle) fn get_autocharge_cycle_count(item: &UItem, effect_key: REffectKey) -> ChargedCycleCount {
    ChargedCycleCount {
        fully_charged: internal_get_autocharge_cycle_count(item, effect_key),
        part_charged: None,
        can_run_uncharged: false,
    }
}

fn internal_get_autocharge_cycle_count(item: &UItem, effect_key: REffectKey) -> InfCount {
    let autocharges = match item.get_autocharges() {
        Some(autocharges) => autocharges,
        // Effect wants autocharge, but item does not support autocharges -> can't cycle
        None => {
            return InfCount::Count(0);
        }
    };
    if !autocharges.contains_ac_for_effect(&effect_key) {
        // Effect wants autocharge, but no autocharge in the item - can't cycle. Since autocharges
        // are not added here when they cannot be loaded, non-loaded autocharges are covered by this
        // as well.
        return InfCount::Count(0);
    };
    // Should always be available, since this method should never be requested for non-loaded items
    let effect_datas = item.get_effect_datas().unwrap();
    match effect_datas.get(&effect_key).unwrap().charge_count {
        Some(charge_count) => InfCount::Count(charge_count),
        None => InfCount::Infinite,
    }
}
