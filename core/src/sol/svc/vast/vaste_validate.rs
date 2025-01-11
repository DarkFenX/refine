use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValFails, SolValOptions, SolVast},
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
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> SolValFails {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
        let mut fail_data = SolValFails::new();
        if options.cpu {
            fail_data.cpu = fit_data.validate_cpu_verbose(uad, calc, fit);
        }
        if options.pg {
            fail_data.pg = fit_data.validate_pg_verbose(uad, calc, fit);
        }
        fail_data
    }
}
