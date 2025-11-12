use itertools::Itertools;

use crate::{def::ItemId, sol::SolarSystem, ud::UItemKey};

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
}
