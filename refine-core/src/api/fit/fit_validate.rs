use crate::{
    Fit,
    api::AffectionDir,
    sol::SolCore,
    stats::StatCtlAffectors,
    svc::vast::{ValOptionsInt, ValResultFitFast},
    ud::UFitId,
    val::{ValOptions, ValResultFit},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn validate_fast(&mut self, options: &ValOptions) -> bool {
        let int_options = ValOptionsInt::from_pub(options, self.sol);
        self.sol.internal_validate_fit_fast(self.uid, &int_options)
    }
    pub fn validate_verbose(&mut self, options: &ValOptions) -> ValResultFit {
        let int_options = ValOptionsInt::from_pub(options, self.sol);
        self.sol.internal_validate_fit_verbose(self.uid, &int_options)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_validate_fit_fast(&mut self, fit_uid: UFitId, options: &ValOptionsInt) -> bool {
        match self.svc.validate_fit_fast(&self.u_data, fit_uid, options) {
            ValResultFitFast::Passed => true,
            ValResultFitFast::Failed => false,
            // Potential drone bandwidth validations which failed on current state are re-considered
            // once more, with modified state
            ValResultFitFast::FailedPdbw => self.internal_validate_fit_pdbw_fast(fit_uid, options),
        }
    }
    pub(in crate::api) fn internal_validate_fit_verbose(
        &mut self,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        let mut result = self.svc.validate_fit_verbose(&self.u_data, fit_uid, options);
        if !result.pdbw_passed() {
            let savepoint = self.internal_tx_savepoint();
            if self.internal_switch_pdbw(fit_uid) {
                self.svc
                    .validate_fit_pdbw_verbose(&self.u_data, fit_uid, options, &mut result);
            }
            self.internal_tx_rollback(savepoint);
        }
        result
    }
    // Not inlined: this path is rarely taken, and with it inlined every fast validation takes a few
    // instructions more
    #[inline(never)]
    fn internal_validate_fit_pdbw_fast(&mut self, fit_uid: UFitId, options: &ValOptionsInt) -> bool {
        let savepoint = self.internal_tx_savepoint();
        let valid =
            self.internal_switch_pdbw(fit_uid) && self.svc.validate_fit_pdbw_fast(&self.u_data, fit_uid, options);
        self.internal_tx_rollback(savepoint);
        valid
    }
    /// Switches off modules which decrease drone bandwidth of the fit's ship. Returns if the ship
    /// got more bandwidth.
    fn internal_switch_pdbw(&mut self, fit_uid: UFitId) -> bool {
        let Some(ship_uid) = self.u_data.fits.get(fit_uid).ship else {
            return false;
        };
        let Some(attr_rid) = self.u_data.r_data.get_attr_consts().drone_bandwidth else {
            return false;
        };
        let Ok(current) = self.internal_get_item_attr(ship_uid, attr_rid) else {
            return false;
        };
        let switched = self.internal_ctl_affectors_switch(
            ship_uid,
            Some(attr_rid),
            StatCtlAffectors::Offline,
            AffectionDir::Decrease,
        );
        if !switched {
            return false;
        }
        self.internal_get_item_attr(ship_uid, attr_rid)
            .is_ok_and(|potential| potential.extra > current.extra)
    }
}
