use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, StanceMut},
    },
    ud::{UEffectUpdates, UFitKey, UItem, UItemKey, UStance},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_stance(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get(fit_key);
        // Remove old stance, if it was set
        if let Some(old_item_key) = u_fit.stance {
            self.internal_remove_stance(old_item_key, reuse_eupdates);
        }
        // Add new stance
        let item_id = self.u_data.items.alloc_id();
        let u_stance = UStance::new(item_id, type_id, fit_key, true, &self.u_data.src);
        let u_item = UItem::Stance(u_stance);
        let item_key = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.stance = Some(item_key);
        SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_stance(&mut self, type_id: ItemTypeId) -> StanceMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.sol.internal_set_fit_stance(self.key, type_id, &mut reuse_eupdates);
        StanceMut::new(self.sol, item_key)
    }
}
