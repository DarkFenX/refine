use crate::svc::cycle::{CycleOptionsSim, CyclingOptions};

pub(super) const CYCLE_OPTIONS_SIM: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim {
    reload_optionals: Some(true),
    ..
});
pub(super) const CYCLE_OPTIONS_BURST: CyclingOptions = CyclingOptions::Burst;
