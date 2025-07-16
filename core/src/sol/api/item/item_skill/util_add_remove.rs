use crate::{
    def::ItemKey,
    sol::SolarSystem,
    svc::Svc,
    uad::{Uad, UadEffectUpdates, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_skill(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        uad_item: &UadItem,
        reuse_eupdates: &UadEffectUpdates,
    ) {
        SolarSystem::util_add_item_without_projs(uad, svc, item_key, uad_item, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_skill(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        uad_item: &UadItem,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        SolarSystem::util_remove_item_without_projs(uad, svc, item_key, uad_item, reuse_eupdates);
    }
}
