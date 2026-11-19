use crate::{
    Sol,
    sol::SolCore,
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
impl SolCore {
    pub(in crate::api) fn internal_validate_fast(&mut self, options: &ValOptionsSolInt) -> bool {
        for &fit_uid in options.fit_uids.iter() {
            if !self.internal_validate_fit_fast(fit_uid, &options.options) {
                return false;
            }
        }
        self.svc.validate_sol_fast(&options.options)
    }
    pub(in crate::api) fn internal_validate_verbose(&mut self, options: &ValOptionsSolInt) -> ValResultSol {
        let mut sol_result = self.svc.validate_sol_verbose(&self.u_data, options);
        for &fit_uid in options.fit_uids.iter() {
            let fit_result = self.internal_validate_fit_verbose(fit_uid, &options.options);
            let fit_id = self.u_data.fits.ext_id_by_int_id(fit_uid);
            sol_result.fits.push((fit_id, fit_result));
        }
        sol_result
    }
}
