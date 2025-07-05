use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::ChargeMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_charge(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_charge = uad_item.get_charge().unwrap();
        let module_key = uad_charge.get_cont_key();
        // Remove outgoing projections
        for projectee_key in uad_charge.get_projs().iter_projectees() {
            // Update services
            let projectee_uad_item = self.uad.items.get(projectee_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                item_key,
                uad_item,
                projectee_key,
                projectee_uad_item,
            );
            // Reverse projections
            self.rprojs.unreg_projectee(&item_key, &projectee_key);
        }
        // Update services
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Update user data
        let uad_module = self.uad.items.get_mut(module_key).get_module_mut().unwrap();
        uad_module.set_charge_key(None);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_charge(self.key);
    }
}
