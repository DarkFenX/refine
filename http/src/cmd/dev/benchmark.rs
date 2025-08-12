use tokio_rayon::rayon::prelude::*;

use crate::cmd::shared::{HSolCloner, HValOptions};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HBenchmarkCmd {
    AttrCalc(HBenchmarkAttrCalcCmd),
    TryFitItems(Box<HBenchmarkTryFitItemsCmd>),
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
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id).unwrap();
        core_fit.benchmark_attr_calc(self.type_id, self.iterations);
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
        let core_options = (&self.validation_options).into();
        let cloner = HSolCloner::new(core_sol);
        let chunk_size = usize::max(
            400,
            (self.type_ids.len() as f64 / tokio_rayon::rayon::current_num_threads() as f64 / 4.0).ceil() as usize,
        );
        self.type_ids.par_chunks(chunk_size).for_each(|chunk| {
            let mut inner_sol = cloner.lock().get();
            let mut inner_fit = inner_sol.get_fit_mut(&self.fit_id).unwrap();
            inner_fit.benchmark_try_items(chunk, &core_options, self.iterations);
            cloner.lock().put(inner_sol);
        });
    }
}
