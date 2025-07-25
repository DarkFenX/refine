use crate::{
    sol::{SolarSystem, api::ProjEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_proj_effect(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Remove outgoing projections
        let u_item = self.u_data.items.get(item_key);
        let u_proj_effect = u_item.get_proj_effect().unwrap();
        for projectee_key in u_proj_effect.get_projs().iter_projectees() {
            let projectee_u_item = self.u_data.items.get(projectee_key);
            SolarSystem::util_remove_item_projection(
                &self.u_data,
                &mut self.svc,
                item_key,
                u_item,
                projectee_key,
                projectee_u_item,
            );
            self.rprojs.unreg_projectee(&item_key, &projectee_key);
        }
        // Remove effect from services
        SolarSystem::util_remove_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        // Remove effect from user data
        self.u_data.proj_effects.remove(&item_key);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_proj_effect(self.key, &mut reuse_eupdates)
    }
}
