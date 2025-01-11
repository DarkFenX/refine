use crate::sol::{
    svc::{calc::SolCalc, vast::SolVastFitData},
    uad::{fit::SolFit, SolUad},
};

impl SolVastFitData {
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_cpu(uad, calc, fit);
        stats.used > stats.output
    }
}
