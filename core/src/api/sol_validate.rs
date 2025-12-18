use crate::{
    sol::SolarSystem,
    svc::vast::{ValOptionsSol, ValOptionsSolInt, ValResultSol},
};

impl SolarSystem {
    pub(in crate::api) fn internal_validate_fast(&mut self, options: &ValOptionsSolInt) -> bool {
        self.svc.validate_sol_fast(&self.u_data, options)
    }
    pub(in crate::api) fn internal_validate_verbose(&mut self, options: &ValOptionsSolInt) -> ValResultSol {
        self.svc.validate_sol_verbose(&self.u_data, options)
    }
    pub fn validate_fast(&mut self, options: &ValOptionsSol) -> bool {
        let int_options = ValOptionsSolInt::from_pub(self, options);
        self.internal_validate_fast(&int_options)
    }
    pub fn validate_verbose(&mut self, options: &ValOptionsSol) -> ValResultSol {
        let int_options = ValOptionsSolInt::from_pub(self, options);
        self.internal_validate_verbose(&int_options)
    }
}
