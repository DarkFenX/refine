use itertools::Itertools;

use crate::sol::{ItemKey, SolarSystem, api::ProjEffectMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_proj_effect(&mut self, item_key: ItemKey) {
        // Remove outgoing projections
        let uad_item = self.uad.items.get(item_key);
        let projectee_item_keys = uad_item
            .get_proj_effect()
            .unwrap()
            .get_projs()
            .iter_projectee_item_keys()
            .copied()
            .collect_vec();
        if !projectee_item_keys.is_empty() {
            for projectee_item_key in projectee_item_keys.into_iter() {
                let projectee_uad_item = self.uad.items.get(projectee_item_key);
                self.svc
                    .remove_item_projection(&self.uad, item_key, uad_item, projectee_item_key, projectee_uad_item);
                self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
            }
            // Clear on-projected effect projections, so that they don't get processed 2nd time on
            // projected effect removal from services
            self.uad
                .items
                .get_mut(item_key)
                .get_proj_effect_mut()
                .unwrap()
                .get_projs_mut()
                .clear();
        }
        // Remove effect from services
        SolarSystem::internal_remove_item_key_from_svc(&self.uad, &mut self.svc, item_key);
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
