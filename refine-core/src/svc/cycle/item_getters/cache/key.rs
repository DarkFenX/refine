use super::item_co_code::ItemCoCode;
use crate::{
    OptionalReload, RearmMinion,
    svc::cycle::CyclingOptions,
    ud::{UItem, UItemId},
};

pub(super) type CacheKey = (UItemId, ItemCoCode);

impl CyclingOptions {
    pub(in crate::svc::cycle::item_getters) fn cache_key(self, item_uid: UItemId, item: &UItem) -> CacheKey {
        // Not all items have all the settings, cache key is aware of that. This brings the risk of
        // someone changing getters and not updating the key, so this has to be maintained through
        // discipline
        let item_co_code = match (self, item) {
            (.., UItem::Drone(..)) | (Self::Burst, ..) => 0,
            (Self::Sim(sim), UItem::Module(..) | UItem::Charge(..)) => 1 + reload_code(sim.optional_reloads),
            (Self::Sim(sim), UItem::Fighter(..) | UItem::Autocharge(..)) => 1 + rearm_code(sim.rearm_minions),
            (Self::Sim(..), ..) => 1,
        };
        (item_uid, ItemCoCode::from_u8(item_co_code))
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
