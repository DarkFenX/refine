use crate::{
    sol::SolarSystem,
    svc::Svc,
    uad::{Uad, UadEffectUpdates, UadItem, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_stance(
        uad: &Uad,
        svc: &mut Svc,
        item_key: UadItemKey,
        reuse_eupdates: &UadEffectUpdates,
    ) {
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(uad, svc, item_key, uad_item, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_stance(
        uad: &Uad,
        svc: &mut Svc,
        item_key: UadItemKey,
        uad_item: &UadItem,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        SolarSystem::util_remove_item_without_projs(uad, svc, item_key, uad_item, reuse_eupdates);
    }
}
