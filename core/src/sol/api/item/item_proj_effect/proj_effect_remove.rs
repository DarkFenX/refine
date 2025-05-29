use crate::sol::{ItemKey, SolarSystem, api::ProjEffectMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_proj_effect(&mut self, item_key: ItemKey) {
        // Remove outgoing projections
        let uad_item = self.uad.items.get(item_key);
        let uad_proj_effect = uad_item.get_proj_effect().unwrap();
        for &projectee_item_key in uad_proj_effect.get_projs().iter_projectee_item_keys() {
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                item_key,
                uad_item,
                projectee_item_key,
                projectee_uad_item,
            );
            self.rprojs.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove effect from services
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Remove effect from user data
        self.uad.proj_effects.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_proj_effect(self.key);
    }
}
