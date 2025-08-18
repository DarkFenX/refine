use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_sw_effect(
        u_data: &mut UData,
        svc: &mut Svc,
        sw_effect_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(sw_effect_key);
        u_item.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_add_item(u_data, svc, sw_effect_key, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_sw_effect(
        u_data: &mut UData,
        svc: &mut Svc,
        sw_effect_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(sw_effect_key);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_remove_item(u_data, svc, sw_effect_key, reuse_eupdates);
    }
}
