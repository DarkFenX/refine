use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValOptions, SolValResult, SolVast},
    },
    uad::{fit::SolFit, SolUad},
};

impl SolVast {
    pub(in crate::sol) fn validate_fit_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> bool {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
        if options.cpu {
            if !fit_data.validate_cpu_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.pg {
            if !fit_data.validate_pg_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.calibration {
            if !fit_data.validate_calibration_fast(uad, calc, fit) {
                return false;
            }
        }
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> SolValResult {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
        let mut result = SolValResult::new();
        if options.cpu {
            result.cpu = fit_data.validate_cpu_verbose(uad, calc, fit);
        }
        if options.pg {
            result.pg = fit_data.validate_pg_verbose(uad, calc, fit);
        }
        if options.calibration {
            result.calibration = fit_data.validate_calibration_verbose(uad, calc, fit);
        }
        result
    }
}
