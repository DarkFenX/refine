use crate::cmd::shared::HValOptions;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HBenchmarkCmd {
    AttrCalc(HBenchmarkAttrCalcCmd),
    TryFitItems(HBenchmarkTryFitItemsCmd),
}
impl HBenchmarkCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        match self {
            Self::AttrCalc(cmd) => cmd.execute(core_sol),
            Self::TryFitItems(cmd) => cmd.execute(core_sol),
        }
    }
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) {
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
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        let core_options = self.validation_options.into_core_val_options(core_sol);
        core_sol.benchmark_try_fit_items(&self.fit_id, &self.type_ids, &core_options, self.iterations);
    }
}
