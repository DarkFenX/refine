use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    sol::{
        SolarSystem,
        api::{FitMut, StanceMut},
    },
    uad::{UadEffectUpdates, UadItem, UadStance},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_stance(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get(fit_key);
        // Remove old stance, if it was set
        if let Some(old_item_key) = uad_fit.stance {
            self.internal_remove_stance(old_item_key, reuse_eupdates);
        }
        // Add new stance
        let item_id = self.uad.items.alloc_id();
        let uad_stance = UadStance::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Stance(uad_stance);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.stance = Some(item_key);
        SolarSystem::util_add_stance(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_stance(&mut self, type_id: ItemTypeId) -> StanceMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_set_fit_stance(self.key, type_id, &mut reuse_eupdates);
        StanceMut::new(self.sol, item_key)
    }
}
