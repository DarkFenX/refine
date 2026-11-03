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
    pub(crate) fn execute(self, core: &mut rc::Core) {
        match self {
            Self::AttrCalc(cmd) => cmd.execute(core),
            Self::CapSim(cmd) => cmd.execute(core),
            Self::Stats(cmd) => cmd.execute(core),
            Self::TryFitItems(cmd) => cmd.execute(core),
        }
    }
}
