use crate::{
    api::ProjEffectMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_proj_effect(&mut self, proj_effect_uid: UItemId) {
        // Remove outgoing projections; effect keeps them, in case it gets reattached
        SolarSystem::util_unreg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, proj_effect_uid);
        // Remove effect from services
        SolarSystem::util_remove_proj_effect(
            &mut self.u_data,
            &mut self.svc,
            proj_effect_uid,
            &mut self.cache.eupdates,
        );
        // Remove effect from user data
        self.u_data.proj_effects.remove(&proj_effect_uid);
        self.internal_discard_item(proj_effect_uid, UndoStep::ProjEffectRemoved);
    }
}

impl<'s> ProjEffectMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_proj_effect(self.uid)
    }
}
