use crate::{
    sol::{SolarSystem, api::SubsystemMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_subsystem(
        &mut self,
        item_key: UadItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_subsystem = uad_item.get_subsystem().unwrap();
        SolarSystem::util_remove_subsystem(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_subsystem.get_fit_key());
        uad_fit.subsystems.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> SubsystemMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_subsystem(self.key, &mut reuse_eupdates)
    }
}
