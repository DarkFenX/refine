use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValOptions, SolVast},
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
}
