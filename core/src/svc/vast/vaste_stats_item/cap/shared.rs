use crate::svc::cycle::{CycleOptionReload, CycleOptions};

pub(super) const CYCLE_OPTIONS_SIM: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Sim,
    reload_optionals: true,
};
pub(super) const CYCLE_OPTIONS_BURST: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};
