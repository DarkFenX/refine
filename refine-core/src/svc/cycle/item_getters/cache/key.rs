use super::item_co_code::ItemCyclingOptionsCode;
use crate::{
    OptionalReload, RearmMinion,
    svc::cycle::item_getters::cycling_options::{DroneCyclingOptions, FighterCyclingOptions, ModuleCyclingOptions},
    ud::UItemId,
};

pub(super) type CacheKey = (UItemId, ItemCyclingOptionsCode);

/// Cycling options an item kind's cycle builder reads, making the cache key with the item UID.
pub(in crate::svc::cycle::item_getters) trait ItemCyclingOptions: Copy {
    fn cache_key(self, item_uid: UItemId) -> CacheKey;
}
/// Cache key of any item for burst cycling, whose options are empty for every item kind.
pub(in crate::svc::cycle::item_getters) fn burst_cache_key(item_uid: UItemId) -> CacheKey {
    (item_uid, ItemCyclingOptionsCode::from_u8(BURST_CODE))
}

// Code of burst options of every item kind; codes of sim options follow it
const BURST_CODE: u8 = 0;

// Variants are destructured without "..", so an option added to them fails to compile until the
// code covers it
impl ItemCyclingOptions for ModuleCyclingOptions {
    fn cache_key(self, item_uid: UItemId) -> CacheKey {
        let item_co_code = match self {
            Self::Burst => BURST_CODE,
            Self::Sim { optional_reloads } => 1 + reload_code(optional_reloads),
        };
        (item_uid, ItemCyclingOptionsCode::from_u8(item_co_code))
    }
}
impl ItemCyclingOptions for FighterCyclingOptions {
    fn cache_key(self, item_uid: UItemId) -> CacheKey {
        let item_co_code = match self {
            Self::Burst => BURST_CODE,
            Self::Sim { rearm_minions } => 1 + rearm_code(rearm_minions),
        };
        (item_uid, ItemCyclingOptionsCode::from_u8(item_co_code))
    }
}
impl ItemCyclingOptions for DroneCyclingOptions {
    fn cache_key(self, item_uid: UItemId) -> CacheKey {
        // Drones read no options, so burst and sim share their sequences
        let Self = self;
        (item_uid, ItemCyclingOptionsCode::from_u8(BURST_CODE))
    }
}

fn reload_code(reload: Option<OptionalReload>) -> u8 {
    match reload {
        None => 0,
        Some(OptionalReload::Disabled) => 1,
        Some(OptionalReload::OnEmpty) => 2,
    }
}
fn rearm_code(rearm: Option<RearmMinion>) -> u8 {
    match rearm {
        None => 0,
        Some(RearmMinion::Disabled) => 1,
        Some(RearmMinion::OnFirstEmpty) => 2,
    }
}
