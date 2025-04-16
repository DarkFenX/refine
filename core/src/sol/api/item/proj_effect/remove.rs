use crate::sol::{ItemKey, SolarSystem, api::ProjEffectMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_proj_effect(&mut self, item_key: ItemKey) {
        // Check if everything is correct
        let uad_item = self.uad.items.get(item_key);
        let uad_proj_effect = uad_item.get_proj_effect().unwrap();
        // Remove outgoing projections
        for &projectee_item_key in uad_proj_effect.get_projs().iter_projectee_item_keys() {
            // Update services
            let uad_projectee_item = self.uad.items.get(projectee_item_key);
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, uad_projectee_item);
            // Update user data - do not update info on projected effect, because projected effect
            // will be discarded anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove effect from services
        self.remove_item_key_from_svc(item_key);
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
