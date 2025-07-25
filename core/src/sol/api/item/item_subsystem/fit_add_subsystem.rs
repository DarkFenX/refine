use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, SubsystemMut},
    },
    ud::{UEffectUpdates, UFitKey, UItem, UItemKey, USubsystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_subsystem(
        &mut self,
        fit_key: UFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_subsystem = USubsystem::new(item_id, a_item_id, fit_key, true, &self.u_data.src, reuse_eupdates);
        let u_item = UItem::Subsystem(u_subsystem);
        let item_key = self.u_data.items.add(u_item);
        u_fit.subsystems.insert(item_key);
        SolarSystem::util_add_subsystem(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_subsystem(&mut self, type_id: ItemTypeId) -> SubsystemMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.sol.internal_add_subsystem(self.key, type_id, &mut reuse_eupdates);
        SubsystemMut::new(self.sol, item_key)
    }
}
