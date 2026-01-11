use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, StanceMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UItem, UItemId, UStance},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_stance(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get(fit_uid);
        // Remove old stance, if it was set
        if let Some(old_stance_uid) = u_fit.stance {
            self.internal_remove_stance(old_stance_uid, reuse_eupdates);
        }
        // Add new stance
        let item_id = self.u_data.items.alloc_id();
        let u_stance = UStance::new(item_id, type_aid, fit_uid, true, &self.u_data.src);
        let u_item = UItem::Stance(u_stance);
        let stance_uid = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.stance = Some(stance_uid);
        SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_uid, reuse_eupdates);
        stance_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn set_stance(&mut self, type_id: ItemTypeId) -> StanceMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let stance_uid = self
            .sol
            .internal_set_fit_stance(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        StanceMut::new(self.sol, stance_uid)
    }
}
