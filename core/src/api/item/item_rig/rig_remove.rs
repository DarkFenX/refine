use crate::{
    api::RigMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_rig(&mut self, rig_key: UItemId, reuse_eupdates: &mut UEffectUpdates) {
        SolarSystem::util_remove_rig(&mut self.u_data, &mut self.svc, rig_key, reuse_eupdates);
        let u_rig = self.u_data.items.get(rig_key).dc_rig().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_rig.get_fit_uid());
        u_fit.rigs.remove(&rig_key);
        self.u_data.items.remove(rig_key);
    }
}

impl<'a> RigMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_rig(self.key, &mut reuse_eupdates)
    }
}
