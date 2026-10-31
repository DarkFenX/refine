use crate::{sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    /// Puts back an autocharge which was replaced by a type change of its parent item.
    pub(in crate::api) fn internal_reattach_autocharge(&mut self, autocharge_uid: UItemId) {
        self.u_data.items.reattach(autocharge_uid);
        let u_autocharge = self.u_data.items.get(autocharge_uid).dc_autocharge().unwrap();
        let cont_item_uid = u_autocharge.get_cont_item_uid();
        let cont_effect_rid = u_autocharge.get_cont_effect_rid();
        let cont_acs = self.u_data.items.get_mut(cont_item_uid).get_autocharges_mut().unwrap();
        cont_acs.set(cont_effect_rid, autocharge_uid);
        SolarSystem::util_reattach_autocharge(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            autocharge_uid,
            &mut self.cache.eupdates,
        );
    }
}
