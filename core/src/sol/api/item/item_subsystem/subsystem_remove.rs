use crate::{
    sol::{SolarSystem, api::SubsystemMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_subsystem(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_subsystem(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_subsystem = self.u_data.items.get(item_key).get_subsystem().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_subsystem.get_fit_key());
        u_fit.subsystems.remove(&item_key);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> SubsystemMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_subsystem(self.key, &mut reuse_eupdates)
    }
}
