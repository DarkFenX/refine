use crate::{
    ad::AItemId,
    api::{ItemTypeId, ProjEffectMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItem, UItemId, UProjEffect},
};

impl SolarSystem {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId) -> ProjEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let proj_effect_uid = self.internal_add_proj_effect(type_id.into_aid(), &mut reuse_eupdates);
        ProjEffectMut::new(self, proj_effect_uid)
    }
    pub(in crate::api) fn internal_add_proj_effect(
        &mut self,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_proj_effect = UProjEffect::new(item_id, type_aid, true, &self.u_data.src);
        let u_item = UItem::ProjEffect(u_proj_effect);
        let proj_effect_uid = self.u_data.items.add(u_item);
        self.u_data.proj_effects.insert(proj_effect_uid);
        SolarSystem::util_add_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_uid, reuse_eupdates);
        proj_effect_uid
    }
}
