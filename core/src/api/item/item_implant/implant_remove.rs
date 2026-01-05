use crate::{
    api::ImplantMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_implant(
        &mut self,
        implant_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_implant(&mut self.u_data, &mut self.svc, implant_key, reuse_eupdates);
        let u_implant = self.u_data.items.get(implant_key).dc_implant().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_implant.get_fit_uid());
        u_fit.implants.remove(&implant_key);
        self.u_data.items.remove(implant_key);
    }
}

impl<'a> ImplantMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_implant(self.key, &mut reuse_eupdates);
    }
}
