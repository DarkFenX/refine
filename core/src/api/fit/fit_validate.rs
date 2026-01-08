use crate::{
    api::FitMut,
    sol::SolarSystem,
    svc::vast::{ValOptions, ValOptionsInt, ValResultFit},
    ud::UFitId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_validate_fit_fast(&mut self, fit_uid: UFitId, options: &ValOptionsInt) -> bool {
        self.svc.validate_fit_fast(&self.u_data, fit_uid, options)
    }
    pub(in crate::api) fn internal_validate_fit_verbose(
        &mut self,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        self.svc.validate_fit_verbose(&self.u_data, fit_uid, options)
    }
}

impl<'a> FitMut<'a> {
    pub fn validate_fast(&mut self, options: &ValOptions) -> bool {
        let int_options = ValOptionsInt::from_pub(self.sol, options);
        self.sol.internal_validate_fit_fast(self.uid, &int_options)
    }
    pub fn validate_verbose(&mut self, options: &ValOptions) -> ValResultFit {
        let int_options = ValOptionsInt::from_pub(self.sol, options);
        self.sol.internal_validate_fit_verbose(self.uid, &int_options)
    }
}
