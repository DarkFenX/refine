use itertools::Itertools;

use crate::{
    def::{AttrVal, ItemId},
    sol::SolarSystem,
    svc::{cycle::Cycle, output::Output},
    ud::UItemKey,
    util::sig_round,
};

pub struct StatCapSimStagger {
    default: bool,
    exception_item_ids: Vec<ItemId>,
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
    pub(super) cycle: Cycle,
    delay: AttrVal,
}
impl StaggerKey {
    pub(super) fn new(cycle: &Cycle, output: &Output<AttrVal>) -> Self {
        // Round everything, so that small differences in result (which is possible due to different
        // order of float operations) end up having the same key
        Self {
            cycle: cycle.copy_rounded(),
            delay: sig_round(output.get_delay(), 10),
        }
    }
}
