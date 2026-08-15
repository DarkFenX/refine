use crate::{FitId, ItemId, dev::DevBenchmarkCmd};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DevBenchmarkStatsCmd {
    fit_id: FitId,
    projectee_item_id: ItemId,
    iterations: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkStatsCmd {
    pub fn new(fit_id: FitId, projectee_item_id: ItemId, iterations: usize) -> Self {
        Self {
            fit_id,
            projectee_item_id,
            iterations,
        }
    }
}
impl From<DevBenchmarkStatsCmd> for DevBenchmarkCmd {
    fn from(sub_cmd: DevBenchmarkStatsCmd) -> Self {
        DevBenchmarkCmd::Stats(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkStatsCmd {
    pub(super) fn execute(self, core_sol: &mut rc::SolarSystem) {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id).unwrap();
        core_fit.benchmark_stats(self.projectee_item_id, self.iterations);
    }
}
