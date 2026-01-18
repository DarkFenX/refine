use itertools::Itertools;
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};
use tokio_rayon::rayon::prelude::*;

use crate::cmd::shared::{HSolCloner, HValOptions};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HBenchmarkCmd {
    AttrCalc(HBenchmarkAttrCalcCmd),
    TryFitItems(Box<HBenchmarkTryFitItemsCmd>),
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HBenchmarkAttrCalcCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    type_id: i32,
    iterations: usize,
}
impl HBenchmarkAttrCalcCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id).unwrap();
        let core_type_id = rc::ItemTypeId::from_i32(self.type_id);
        core_fit.benchmark_attr_calc(core_type_id, self.iterations);
    }
}

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HBenchmarkTryFitItemsCmd {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    type_ids: Vec<i32>,
    validation_options: HValOptions,
    iterations: usize,
}
impl HBenchmarkTryFitItemsCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        let core_options = self.validation_options.to_core();
        let core_type_ids = self.type_ids.iter().map(|v| rc::ItemTypeId::from_i32(*v)).collect_vec();
        let cloner = HSolCloner::new(core_sol);
        let chunk_size = usize::max(
            400,
            (core_type_ids.len() as f64 / tokio_rayon::rayon::current_num_threads() as f64 / 4.0).ceil() as usize,
        );
        core_type_ids.par_chunks(chunk_size).for_each(|chunk| {
            let mut inner_sol = cloner.lock().get();
            let mut inner_fit = inner_sol.get_fit_mut(&self.fit_id).unwrap();
            inner_fit.benchmark_try_items(chunk, &core_options, self.iterations);
            cloner.lock().put(inner_sol);
        });
    }
}
