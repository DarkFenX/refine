use crate::sol::{ItemKey, SolarSystem, proj_tracker::ProjTracker, svc::Svc, uad::Uad};

impl SolarSystem {
    pub(in crate::sol::api) fn load_fighter(
        svc: &mut Svc,
        uad: &mut Uad,
        proj_tracker: &mut ProjTracker,
        item_key: ItemKey,
    ) {
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        svc.load_item(uad, item_key, uad_item);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(svc, uad, proj_tracker, item_key);
    }
    pub(in crate::sol::api) fn unload_fighter(
        svc: &mut Svc,
        uad: &mut Uad,
        proj_tracker: &mut ProjTracker,
        item_key: ItemKey,
    ) {
        // Process autocharges
        SolarSystem::remove_fighter_autocharges(svc, uad, proj_tracker, item_key);
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        svc.unload_item(uad, item_key, uad_item);
    }
}
