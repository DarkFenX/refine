use crate::sol::{
    FitKey, SolarSystem,
    api::FitMut,
    svc::vast::{ValOptions, ValOptionsInt, ValResultFit},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_validate_fit_fast(&mut self, fit_key: FitKey, options: &ValOptionsInt) -> bool {
        self.svc.validate_fit_fast(&self.uad, &self.reffs, fit_key, options)
    }
    pub(in crate::sol::api) fn internal_validate_fit_verbose(
        &mut self,
        fit_key: FitKey,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        self.svc.validate_fit_verbose(&self.uad, &self.reffs, fit_key, options)
    }
}

impl<'a> FitMut<'a> {
    pub fn validate_fast(&mut self, options: &ValOptions) -> bool {
        let int_options = ValOptionsInt::from_pub(self.sol, options);
        self.sol.internal_validate_fit_fast(self.key, &int_options)
    }
    pub fn validate_verbose(&mut self, options: &ValOptions) -> ValResultFit {
        let int_options = ValOptionsInt::from_pub(self.sol, options);
        self.sol.internal_validate_fit_verbose(self.key, &int_options)
    }
}
