use crate::{FitId, dev::DevBenchmarkCmd, stats::StatCapSimStagger};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DevBenchmarkCapSimCmd {
    fit_id: FitId,
    staggers: Vec<StatCapSimStagger>,
    iterations: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkCapSimCmd {
    pub fn new(fit_id: FitId, staggers: Vec<StatCapSimStagger>, iterations: usize) -> Self {
        Self {
            fit_id,
            staggers,
            iterations,
        }
    }
}
impl From<DevBenchmarkCapSimCmd> for DevBenchmarkCmd {
    fn from(sub_cmd: DevBenchmarkCapSimCmd) -> Self {
        DevBenchmarkCmd::CapSim(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkCapSimCmd {
    pub(super) fn execute(self, core_sol: &mut rc::SolarSystem) {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id).unwrap();
        core_fit.benchmark_cap_sim(&self.staggers, self.iterations);
    }
}
