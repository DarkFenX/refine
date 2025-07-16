use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::ProjEffectMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_proj_effect(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        // Remove outgoing projections
        let uad_item = self.uad.items.get(item_key);
        let uad_proj_effect = uad_item.get_proj_effect().unwrap();
        for projectee_key in uad_proj_effect.get_projs().iter_projectees() {
            let projectee_uad_item = self.uad.items.get(projectee_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                item_key,
                uad_item,
                projectee_key,
                projectee_uad_item,
            );
            self.rprojs.unreg_projectee(&item_key, &projectee_key);
        }
        // Remove effect from services
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        // Remove effect from user data
        self.uad.proj_effects.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_proj_effect(self.key, &mut reuse_eupdates)
    }
}
