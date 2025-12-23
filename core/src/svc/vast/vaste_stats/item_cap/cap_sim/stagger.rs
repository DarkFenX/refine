use itertools::Itertools;

use super::aggregate::Aggregator;
use crate::{
    def::{AttrVal, ItemId, OF},
    sol::SolarSystem,
    svc::{
        cycle::{Cycle, CycleDataTime, CycleDataTimeCharged},
        output::Output,
    },
    ud::UItemKey,
    util::{RMapVec, sig_round},
};

pub struct StatCapSimStagger {
    pub default: bool,
    pub exception_item_ids: Vec<ItemId>,
}
impl StatCapSimStagger {
    pub fn new(default: bool) -> Self {
        Self {
            default,
            exception_item_ids: Vec::new(),
        }
    }
}

pub(crate) struct StatCapSimStaggerInt {
    default: bool,
    exception_item_keys: Vec<UItemKey>,
}
impl StatCapSimStaggerInt {
    pub(crate) fn from_pub(sol: &SolarSystem, pub_opts: &StatCapSimStagger) -> Self {
        Self {
            default: pub_opts.default,
            exception_item_keys: pub_opts
                .exception_item_ids
                .iter()
                .filter_map(|fit_id| sol.u_data.items.key_by_id(fit_id))
                .unique()
                .collect(),
        }
    }
    pub(in crate::svc::vast) fn is_staggered(&self, item_key: UItemKey) -> bool {
        self.default ^ self.exception_item_keys.contains(&item_key)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct StaggerKey {
    // Out of all cycle parameters, only time between cycles is used to decide what goes into a
    // stagger group
    pub(super) cycle: Cycle<CycleDataTime>,
    delay: AttrVal,
}
impl StaggerKey {
    pub(super) fn new(cycle: &Cycle<CycleDataTime>, output: &Output<AttrVal>) -> Self {
        // Round everything, so that small differences in result (which is possible due to different
        // order of float operations) end up having the same key
        Self {
            cycle: cycle.copy_rounded(),
            delay: sig_round(output.get_delay(), 10),
        }
    }
}

pub(super) fn process_staggers(
    stagger_map: RMapVec<StaggerKey, (Cycle<CycleDataTimeCharged>, Output<AttrVal>)>,
    aggregator: &mut Aggregator,
) {
    for (stagger_key, stagger_group) in stagger_map.into_iter() {
        if stagger_group.len() < 2 {
            for (cycle, output) in stagger_group.into_iter() {
                aggregator.add_entry(OF(0.0), cycle, output);
            }
            continue;
        }
        // Sort by output value, from highest to lowest
        let stagger_period = stagger_key.cycle.get_first().time / stagger_group.len() as f64;
        for (i, (cycle, output)) in stagger_group
            .into_iter()
            .sorted_by_key(|(_, o)| -o.absolute_impact())
            .enumerate()
        {
            aggregator.add_entry(stagger_period * i as f64, cycle, output)
        }
    }
}
