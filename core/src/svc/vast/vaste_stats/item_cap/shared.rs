use crate::svc::cycle::{CycleOptions, CycleOptionsSim};

pub(super) const CYCLE_OPTIONS_SIM: CycleOptions = CycleOptions::Sim(CycleOptionsSim {
    reload_optionals: Some(true),
    ..
});
pub(super) const CYCLE_OPTIONS_BURST: CycleOptions = CycleOptions::Burst;
