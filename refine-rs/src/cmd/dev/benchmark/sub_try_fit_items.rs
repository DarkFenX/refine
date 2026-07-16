use tokio_rayon::rayon::prelude::*;

use crate::{FitId, ItemTypeId, cmd::shared::SolCloner, dev::DevBenchmarkCmd, val::ValOptions};

pub struct DevBenchmarkTryFitItemsCmd {
    fit_id: FitId,
    type_ids: Vec<ItemTypeId>,
    val_options: ValOptions,
    iterations: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkTryFitItemsCmd {
    pub fn new(fit_id: FitId, type_ids: Vec<ItemTypeId>, val_options: ValOptions, iterations: usize) -> Self {
        Self {
            fit_id,
            type_ids,
            val_options,
            iterations,
        }
    }
}
impl From<DevBenchmarkTryFitItemsCmd> for DevBenchmarkCmd {
    fn from(sub_cmd: DevBenchmarkTryFitItemsCmd) -> Self {
        DevBenchmarkCmd::TryFitItems(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkTryFitItemsCmd {
    pub(super) fn execute(self, core_sol: &mut rc::SolarSystem) {
        let cloner = SolCloner::new(core_sol);
        let chunk_size = usize::max(
            400,
            (self.type_ids.len() as f64 / tokio_rayon::rayon::current_num_threads() as f64 / 4.0).ceil() as usize,
        );
        self.type_ids.par_chunks(chunk_size).for_each(|chunk| {
            let mut inner_sol = cloner.lock().get();
            let mut inner_fit = inner_sol.get_fit_mut(&self.fit_id).unwrap();
            inner_fit.benchmark_try_items(chunk, &self.val_options, self.iterations);
            cloner.lock().put(inner_sol);
        });
    }
}
