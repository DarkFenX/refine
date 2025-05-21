use crate::sol::{
    ItemKey, SolarSystem,
    running_effects::RunningEffects,
    svc::Svc,
    uad::{Uad, item::UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn load_booster(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut RunningEffects,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        SolarSystem::util_load_item(uad, svc, reffs, item_key, uad_item);
    }
    pub(in crate::sol::api) fn unload_booster(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut RunningEffects,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        SolarSystem::util_unload_item(uad, svc, reffs, item_key, uad_item);
    }
}
