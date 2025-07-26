use crate::{
    sol::{SolarSystem, api::ChargeMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_charge(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        let u_charge = u_item.get_charge().unwrap();
        let module_key = u_charge.get_cont_item_key();
        // Remove outgoing projections
        for projectee_key in u_charge.get_projs().iter_projectees() {
            // Update services
            let projectee_u_item = self.u_data.items.get(projectee_key);
            SolarSystem::util_remove_item_projection(
                &self.u_data,
                &mut self.svc,
                item_key,
                u_item,
                projectee_key,
                projectee_u_item,
            );
            // Reverse projections
            self.rev_projs.unreg_projectee(&item_key, &projectee_key);
        }
        // Update services
        SolarSystem::util_remove_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        // Update user data
        let u_module = self.u_data.items.get_mut(module_key).get_module_mut().unwrap();
        u_module.set_charge_key(None);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_charge(self.key, &mut reuse_eupdates)
    }
}
