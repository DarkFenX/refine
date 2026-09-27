use crate::{api::SubsystemMut, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_subsystem(&mut self, subsystem_uid: UItemId) {
        SolarSystem::util_remove_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, &mut self.cache.eupdates);
        let u_subsystem = self.u_data.items.get(subsystem_uid).dc_subsystem().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_subsystem.get_fit_uid());
        u_fit.subsystems.remove(&subsystem_uid);
        self.u_data.items.remove(subsystem_uid);
    }
}

impl<'s> SubsystemMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_subsystem(self.uid)
    }
}
