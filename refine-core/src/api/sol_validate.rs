use crate::{
    Sol,
    sol::CoreInner,
    svc::vast::ValOptionsSolInt,
    val::{ValOptionsSol, ValResultSol},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn validate_fast(&mut self, options: &ValOptionsSol) -> bool {
        let sol = self.inner_mut();
        let int_options = ValOptionsSolInt::from_pub(options, sol);
        sol.internal_validate_fast(&int_options)
    }
    pub fn validate_verbose(&mut self, options: &ValOptionsSol) -> ValResultSol {
        let sol = self.inner_mut();
        let int_options = ValOptionsSolInt::from_pub(options, sol);
        sol.internal_validate_verbose(&int_options)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_validate_fast(&mut self, options: &ValOptionsSolInt) -> bool {
        self.svc.validate_sol_fast(&self.u_data, options)
    }
    pub(in crate::api) fn internal_validate_verbose(&mut self, options: &ValOptionsSolInt) -> ValResultSol {
        self.svc.validate_sol_verbose(&self.u_data, options)
    }
}
