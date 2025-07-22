use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, SubsystemMut},
    },
    uad::{UadEffectUpdates, UadFitKey, UadItem, UadItemKey, UadSubsystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_subsystem(
        &mut self,
        fit_key: UadFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> UadItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_subsystem = UadSubsystem::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Subsystem(uad_subsystem);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.subsystems.insert(item_key);
        SolarSystem::util_add_subsystem(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_subsystem(&mut self, type_id: ItemTypeId) -> SubsystemMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_add_subsystem(self.key, type_id, &mut reuse_eupdates);
        SubsystemMut::new(self.sol, item_key)
    }
}
