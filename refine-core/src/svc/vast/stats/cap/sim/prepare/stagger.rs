use itertools::Itertools;

use super::{
    merge::Merger,
    timing_key::{CSeqHardDtTimingKey, CSeqPartTimingKey},
};
use crate::{
    ItemId, PValue, SolarSystem,
    svc::{
        cycle::CycleSeq,
        vast::{aggr::AggrIterData, stats::cap::sim::shared::Direction},
    },
    ud::UItemId,
    util::RMapVec,
};

#[derive(Clone)]
pub struct StatCapSimStagger<I = ItemId> {
    default: bool = false,
    exception_item_ids: Vec<I> = Vec::new(),
}
const impl<I> Default for StatCapSimStagger<I> {
    fn default() -> Self {
        Self { .. }
    }
}
impl<I> StatCapSimStagger<I> {
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    pub fn with_exception_item_ids(mut self, exception_item_ids: impl Iterator<Item = I>) -> Self {
        self.exception_item_ids.extend(exception_item_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I1> StatCapSimStagger<I1> {
    pub fn try_map_ids<I2, E, M>(self, mut item_mapper: M) -> Result<StatCapSimStagger<I2>, E>
    where
        M: FnMut(I1) -> Result<I2, E>,
    {
        let mut exception_item_ids = Vec::with_capacity(self.exception_item_ids.len());
        for exception_item_id in self.exception_item_ids {
            exception_item_ids.push(item_mapper(exception_item_id)?);
        }
        Ok(StatCapSimStagger {
            default: self.default,
            exception_item_ids,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Internal representation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
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
                .filter_map(|fit_id| sol.u_data.items.int_id_by_ext_id(fit_id))
                .unique()
                .collect(),
        }
    }
    pub(super) fn is_staggered(&self, item_uid: UItemId) -> bool {
        self.default ^ self.exception_item_uids.contains(&item_uid)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct StaggerKey {
    cseq: CycleSeq<CSeqPartTimingKey, CSeqHardDtTimingKey>,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de, I> Deserialize<'de> for StatCapSimStagger<I>
    where
        I: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match StatCapSimStaggerFormats::deserialize(deserializer)? {
                StatCapSimStaggerFormats::Simple(default) => StatCapSimStagger { default, .. },
                StatCapSimStaggerFormats::Extended(default, exception_item_ids) => StatCapSimStagger {
                    default,
                    exception_item_ids,
                },
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StatCapSimStaggerFormats<I> {
        Simple(bool),
        Extended(bool, Vec<I>),
    }
}
