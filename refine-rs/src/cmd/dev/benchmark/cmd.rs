use crate::dev::{DevBenchmarkAttrCalcCmd, DevBenchmarkStatsCmd, DevBenchmarkTryFitItemsCmd};

pub enum DevBenchmarkCmd {
    AttrCalc(DevBenchmarkAttrCalcCmd),
    Stats(DevBenchmarkStatsCmd),
    TryFitItems(DevBenchmarkTryFitItemsCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) {
        match self {
            Self::AttrCalc(cmd) => cmd.execute(core_sol),
            Self::Stats(cmd) => cmd.execute(core_sol),
            Self::TryFitItems(cmd) => cmd.execute(core_sol),
        }
    }
}
