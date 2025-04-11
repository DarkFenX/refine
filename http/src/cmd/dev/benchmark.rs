use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};
use tokio_rayon::rayon::prelude::*;

use crate::cmd::shared::HValOptions;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HBenchmarkCmd {
    AttrCalc(HBenchmarkAttrCalcCmd),
    TryFitItems(HBenchmarkTryFitItemsCmd),
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HBenchmarkAttrCalcCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    type_id: rc::ItemTypeId,
    iterations: usize,
}
impl HBenchmarkAttrCalcCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        core_sol.benchmark_attr_calc(&self.fit_id, self.type_id, self.iterations);
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HBenchmarkTryFitItemsCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    type_ids: Vec<rc::ItemTypeId>,
    validation_options: HValOptions,
    iterations: usize,
}
impl HBenchmarkTryFitItemsCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        let core_options = self.validation_options.to_core_val_options(core_sol);
        let mut breeder = HGuardedSolBreeder::new(core_sol);
        let chunk_size = usize::max(
            400,
            (self.type_ids.len() as f64 / tokio_rayon::rayon::current_num_threads() as f64 / 4.0).ceil() as usize,
        );
        self.type_ids.par_chunks(chunk_size).for_each(|chunk| {
            let mut inner_sol = breeder.lock().get();
            inner_sol.benchmark_try_fit_items(&self.fit_id, chunk, &core_options, self.iterations);
            breeder.lock().put(inner_sol);
        });
    }
}

struct HGuardedSolBreeder<'a> {
    h_breeder: Arc<Mutex<HSolBreeder<'a>>>,
}
impl<'a> HGuardedSolBreeder<'a> {
    pub(crate) fn new(original: &'a rc::SolarSystem) -> Self {
        Self {
            h_breeder: Arc::new(Mutex::new(HSolBreeder::new(original))),
        }
    }
    pub(crate) fn lock(&'a self) -> MutexGuard<HSolBreeder> {
        self.h_breeder.lock()
    }
}

struct HSolBreeder<'a> {
    original: &'a rc::SolarSystem,
    allocated: Vec<rc::SolarSystem>,
}
impl<'a> HSolBreeder<'a> {
    fn new(original: &'a rc::SolarSystem) -> Self {
        Self {
            original,
            allocated: Vec::new(),
        }
    }
    fn get(&mut self) -> rc::SolarSystem {
        match self.allocated.pop() {
            Some(sol) => sol,
            None => self.original.clone(),
        }
    }
    fn put(&mut self, sol: rc::SolarSystem) {
        self.allocated.push(sol);
    }
}
