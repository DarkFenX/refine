use tokio_rayon::rayon::prelude::*;

use crate::{
    FitId, ItemTypeId,
    dev::{DevBenchmarkCmd, SolCloner},
    val::ValOptions,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
        DevBenchmarkCmd::TryFitItems(Box::new(sub_cmd))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkTryFitItemsCmd {
    pub(super) fn execute(self, core: &mut rc::Core) {
        let cloner = SolCloner::new(core);
        let chunk_size = usize::max(
            100,
            (self.type_ids.len() as f64 / tokio_rayon::rayon::current_num_threads() as f64 / 16.0).ceil() as usize,
        );
        // with_max_len() ensures each chunk goes to a separate rayon worker
        self.type_ids.par_chunks(chunk_size).with_max_len(1).for_each(|chunk| {
            let mut inner_core = cloner.lock().take();
            let mut inner_fit = inner_core.get_fit(&self.fit_id).unwrap();
            inner_fit.benchmark_try_items(chunk, &self.val_options, self.iterations);
            cloner.lock().put(inner_core);
        });
    }
}
