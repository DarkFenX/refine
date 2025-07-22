use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, RigMut},
    },
    uad::{UadEffectUpdates, UadFitKey, UadItem, UadItemKey, UadRig},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_rig(
        &mut self,
        fit_key: UadFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> UadItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_rig = UadRig::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Rig(uad_rig);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.rigs.insert(item_key);
        SolarSystem::util_add_rig(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_rig(&mut self, type_id: ItemTypeId) -> RigMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_add_rig(self.key, type_id, &mut reuse_eupdates);
        RigMut::new(self.sol, item_key)
    }
}
