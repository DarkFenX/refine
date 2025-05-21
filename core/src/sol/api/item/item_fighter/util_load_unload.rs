use crate::sol::{ItemKey, SolarSystem, reffs::REffs, rprojs::RProjs, svc::Svc, uad::Uad};

impl SolarSystem {
    pub(in crate::sol::api) fn load_fighter(
        uad: &mut Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        rprojs: &mut RProjs,
        item_key: ItemKey,
    ) {
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_load_item(uad, svc, reffs, item_key, uad_item);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(uad, svc, reffs, rprojs, item_key);
    }
    pub(in crate::sol::api) fn unload_fighter(
        uad: &mut Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        rprojs: &mut RProjs,
        item_key: ItemKey,
    ) {
        // Process autocharges
        SolarSystem::remove_fighter_autocharges(uad, svc, reffs, rprojs, item_key, true);
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_unload_item(uad, svc, reffs, item_key, uad_item);
    }
}
