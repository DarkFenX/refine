use itertools::Itertools;

use super::aggregate::Aggregator;
use crate::{
    def::{AttrVal, ItemId, OF},
    sol::SolarSystem,
    svc::{
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleSeq},
        output::Output,
    },
    ud::UItemId,
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
    exception_item_keys: Vec<UItemId>,
}
impl StatCapSimStaggerInt {
    pub(crate) fn from_pub(sol: &SolarSystem, pub_opts: &StatCapSimStagger) -> Self {
        Self {
            default: pub_opts.default,
            exception_item_keys: pub_opts
                .exception_item_ids
                .iter()
                .filter_map(|fit_id| sol.u_data.items.iid_by_eid(fit_id))
                .unique()
                .collect(),
        }
    }
    pub(in crate::svc::vast) fn is_staggered(&self, item_key: UItemId) -> bool {
        self.default ^ self.exception_item_keys.contains(&item_key)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct StaggerKey {
    // Out of all cycle parameters, only time between cycles is used to decide what goes into a
    // stagger group
    pub(super) cseq: CycleSeq<CycleDataTime>,
    delay: AttrVal,
}
impl StaggerKey {
    pub(super) fn new(cseq: &CycleSeq<CycleDataTime>, opc: &Output<AttrVal>) -> Self {
        // Round everything, so that small differences in result (which is possible due to different
        // order of float operations) end up having the same key
        Self {
            cseq: cseq.copy_rounded(),
            delay: match opc.iter_amounts().next() {
                Some(output_event) => sig_round(output_event.time, 10),
                None => OF(0.0),
            },
        }
    }
}

pub(super) fn process_staggers(
    stagger_map: RMapVec<StaggerKey, (CycleSeq<CycleDataTimeCharge>, Output<AttrVal>)>,
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
        let stagger_period = stagger_key.cseq.get_first_cycle().time / stagger_group.len() as f64;
        for (i, (cycle, output)) in stagger_group
            .into_iter()
            .sorted_by_key(|(_, o)| -o.absolute_impact())
            .enumerate()
        {
            aggregator.add_entry(stagger_period * i as f64, cycle, output)
        }
    }
}
