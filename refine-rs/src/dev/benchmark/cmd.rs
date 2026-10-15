use crate::dev::{DevBenchmarkAttrCalcCmd, DevBenchmarkCapSimCmd, DevBenchmarkStatsCmd, DevBenchmarkTryFitItemsCmd};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum DevBenchmarkCmd {
    AttrCalc(DevBenchmarkAttrCalcCmd),
    CapSim(DevBenchmarkCapSimCmd),
    Stats(DevBenchmarkStatsCmd),
    TryFitItems(Box<DevBenchmarkTryFitItemsCmd>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) {
        match self {
            Self::AttrCalc(cmd) => cmd.execute(core_sol),
            Self::CapSim(cmd) => cmd.execute(core_sol),
            Self::Stats(cmd) => cmd.execute(core_sol),
            Self::TryFitItems(cmd) => cmd.execute(core_sol),
        }
    }
}
