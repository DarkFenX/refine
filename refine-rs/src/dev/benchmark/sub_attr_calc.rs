use crate::{FitId, ItemTypeId, dev::DevBenchmarkCmd};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DevBenchmarkAttrCalcCmd {
    fit_id: FitId,
    type_id: ItemTypeId,
    iterations: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkAttrCalcCmd {
    pub fn new(fit_id: FitId, type_id: ItemTypeId, iterations: usize) -> Self {
        Self {
            fit_id,
            type_id,
            iterations,
        }
    }
}
impl From<DevBenchmarkAttrCalcCmd> for DevBenchmarkCmd {
    fn from(sub_cmd: DevBenchmarkAttrCalcCmd) -> Self {
        DevBenchmarkCmd::AttrCalc(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkAttrCalcCmd {
    pub(super) fn execute(self, core_sol: &mut rc::SolarSystem) {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id).unwrap();
        core_fit.benchmark_attr_calc(self.type_id, self.iterations);
    }
}
