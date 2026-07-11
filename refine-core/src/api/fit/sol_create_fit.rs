use crate::{
    api::FitMut,
    sol::SolarSystem,
    ud::{UFit, UFitId},
};

impl SolarSystem {
    pub fn create_fit(&mut self) -> FitMut<'_> {
        let fit_uid = self.create_fit_internal();
        FitMut::new(self, fit_uid)
    }
    pub(in crate::api) fn create_fit_internal(&mut self) -> UFitId {
        let fit_id = self.u_data.fits.alloc_id();
        let u_fit = UFit::new(fit_id);
        let fit_uid = self.u_data.fits.add(u_fit);
        self.svc.notify_fit_added(fit_uid);
        fit_uid
    }
}
