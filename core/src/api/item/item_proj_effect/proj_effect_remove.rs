use crate::{
    api::ProjEffectMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_proj_effect(
        &mut self,
        proj_effect_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Remove outgoing projections
        let u_proj_effect = self.u_data.items.get(proj_effect_uid).dc_proj_effect().unwrap();
        if !u_proj_effect.get_projs().is_empty() {
            for projectee_uid in u_proj_effect.get_projs().iter_projectees() {
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, proj_effect_uid, projectee_uid);
                self.rev_projs.unreg_projectee(&proj_effect_uid, projectee_uid);
            }
            let u_proj_effect = self.u_data.items.get_mut(proj_effect_uid).dc_proj_effect_mut().unwrap();
            u_proj_effect.get_projs_mut().clear();
        }
        // Remove effect from services
        SolarSystem::util_remove_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_uid, reuse_eupdates);
        // Remove effect from user data
        self.u_data.proj_effects.remove(&proj_effect_uid);
        self.u_data.items.remove(proj_effect_uid);
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_proj_effect(self.uid, &mut reuse_eupdates)
    }
}
