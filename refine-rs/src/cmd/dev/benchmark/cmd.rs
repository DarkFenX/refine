use crate::dev::BenchmarkAttrCalcCmd;

pub enum DevBenchmarkCmd {
    AttrCalc(BenchmarkAttrCalcCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DevBenchmarkCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) {
        match self {
            Self::AttrCalc(cmd) => cmd.execute(core_sol),
        }
    }
}
