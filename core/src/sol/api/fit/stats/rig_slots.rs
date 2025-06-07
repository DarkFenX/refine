use crate::sol::{FitKey, SolarSystem, api::FitMut, svc::vast::StatSlot};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_stat_fit_rig_slots(&mut self, fit_key: FitKey) -> StatSlot {
        let fit = self.uad.fits.get(fit_key);
        self.svc
            .vast
            .get_fit_data(&fit_key)
            .get_stat_rig_slots(&self.uad, &mut self.svc.calc, fit)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_stat_rig_slots(&mut self) -> StatSlot {
        self.sol.internal_get_stat_fit_rig_slots(self.key)
    }
}
