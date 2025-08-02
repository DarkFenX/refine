use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_process_charge_activation(
        u_data: &mut UData,
        svc: &mut Svc,
        charge_key: UItemKey,
        activated: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_charge = u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
        let old_a_state = u_charge.get_state();
        u_charge.set_activated(activated);
        let new_a_state = u_charge.get_state();
        u_charge.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_switch_item_state(u_data, svc, charge_key, old_a_state, new_a_state, reuse_eupdates);
    }
}
