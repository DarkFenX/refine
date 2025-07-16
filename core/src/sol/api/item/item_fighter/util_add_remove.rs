use crate::{
    def::ItemKey,
    sol::{SolarSystem, rprojs::RProjs},
    svc::Svc,
    uad::{Uad, UadEffectUpdates},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_fighter_with_projs(
        uad: &mut Uad,
        svc: &mut Svc,
        rprojs: &mut RProjs,
        item_key: ItemKey,
        reuse_eupdates: &UadEffectUpdates,
    ) {
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_add_item_with_projs(uad, svc, item_key, uad_item, reuse_eupdates);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(uad, svc, rprojs, item_key);
    }
    pub(in crate::sol::api) fn util_remove_fighter_with_projs(
        uad: &mut Uad,
        svc: &mut Svc,
        rprojs: &mut RProjs,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        // Process autocharges
        SolarSystem::remove_fighter_autocharges(uad, svc, rprojs, item_key, true, reuse_eupdates);
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_remove_item_with_projs(uad, svc, item_key, uad_item, reuse_eupdates);
    }
}
