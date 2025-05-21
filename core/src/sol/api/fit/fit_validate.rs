use crate::sol::{
    FitKey, SolarSystem,
    api::FitMut,
    svc::vast::{ValOptions, ValResult},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_validate_fit_fast(&mut self, fit_key: FitKey, options: &ValOptions) -> bool {
        self.svc
            .vast
            .validate_fit_fast(&self.uad, &mut self.svc.calc, &self.reffs, fit_key, options)
    }
    pub(in crate::sol::api) fn internal_validate_fit_verbose(
        &mut self,
        fit_key: FitKey,
        options: &ValOptions,
    ) -> ValResult {
        self.svc
            .vast
            .validate_fit_verbose(&self.uad, &mut self.svc.calc, &self.reffs, fit_key, options)
    }
}

impl<'a> FitMut<'a> {
    pub fn validate_fast(&mut self, options: &ValOptions) -> bool {
        self.sol.internal_validate_fit_fast(self.key, options)
    }
    pub fn validate_verbose(&mut self, options: &ValOptions) -> ValResult {
        self.sol.internal_validate_fit_verbose(self.key, options)
    }
}
