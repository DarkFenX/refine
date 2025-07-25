use crate::{
    sol::{SolarSystem, api::FitMut},
    ud::{UFit, UFitKey},
};

impl SolarSystem {
    pub fn add_fit(&mut self) -> FitMut<'_> {
        let fit_key = self.add_fit_internal();
        FitMut::new(self, fit_key)
    }
    pub(in crate::sol::api) fn add_fit_internal(&mut self) -> UFitKey {
        let fit_id = self.u_data.fits.alloc_id();
        let u_fit = UFit::new(fit_id);
        let fit_key = self.u_data.fits.add(u_fit);
        self.svc.notify_fit_added(fit_key);
        fit_key
    }
}
