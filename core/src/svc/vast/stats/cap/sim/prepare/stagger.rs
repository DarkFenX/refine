use itertools::Itertools;

use super::{key::CSeqPartTimingKey, merge::Merger};
use crate::{
    num::PValue,
    sol::SolarSystem,
    svc::{
        cycle::CycleSeq,
        vast::{aggr::AggrIterData, stats::cap::sim::shared::Direction},
    },
    ud::{ItemId, UItemId},
    util::RMapVec,
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
    exception_item_uids: Vec<UItemId>,
}
impl StatCapSimStaggerInt {
    pub(crate) fn from_pub(sol: &SolarSystem, pub_opts: &StatCapSimStagger) -> Self {
        Self {
            default: pub_opts.default,
            exception_item_uids: pub_opts
                .exception_item_ids
                .iter()
                .filter_map(|fit_id| sol.u_data.items.iid_by_xid(fit_id))
                .unique()
                .collect(),
        }
    }
    pub(in crate::svc::vast) fn is_staggered(&self, item_uid: UItemId) -> bool {
        self.default ^ self.exception_item_uids.contains(&item_uid)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct StaggerKey {
    cseq: CycleSeq<CSeqPartTimingKey>,
}
impl StaggerKey {
    pub(super) fn new(iter_data: &AggrIterData<PValue>) -> Self {
        Self {
            cseq: iter_data.extract_cseq_timing_key(),
        }
    }
}

pub(super) fn process_staggers(
    stagger_map: RMapVec<StaggerKey, AggrIterData<PValue>>,
    merger: &mut Merger,
    direction: Direction,
) {
    for (stagger_key, stagger_group) in stagger_map.into_iter() {
        if stagger_group.len() < 2 {
            for iter_data in stagger_group.into_iter() {
                merger.add_entry(PValue::ZERO, iter_data, direction);
            }
            continue;
        }
        // Sort by first seen instance value, from highest to lowest
        let stagger_period = stagger_key.cseq.get_first_cycle().duration / PValue::from_usize(stagger_group.len());
        for (i, iter_data) in stagger_group
            .into_iter()
            .sorted_by_key(|cseq_iter_data| {
                cseq_iter_data
                    .iter()
                    .next()
                    .and_then(|cseq_iter_item| cseq_iter_item.output.into_instance_iter().next())
                    .map(|instance_iter_item| instance_iter_item.instance)
                    .unwrap_or(PValue::ZERO)
            })
            .enumerate()
        {
            merger.add_entry(stagger_period * PValue::from_usize(i), iter_data, direction)
        }
    }
}
