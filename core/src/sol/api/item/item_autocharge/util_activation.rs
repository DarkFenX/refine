use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_process_autocharge_activations(
        u_data: &mut UData,
        svc: &mut Svc,
        ac_activations: Vec<(UItemKey, bool)>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        for (ac_key, ac_activated) in ac_activations {
            let u_autocharge = u_data.items.get_mut(ac_key).get_autocharge_mut().unwrap();
            let old_a_state = u_autocharge.get_state();
            u_autocharge.set_activated(ac_activated);
            let new_a_state = u_autocharge.get_state();
            u_autocharge.update_reffs(reuse_eupdates, &u_data.src);
            SolarSystem::util_switch_item_state(u_data, svc, ac_key, old_a_state, new_a_state, reuse_eupdates);
        }
    }
}
