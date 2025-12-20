use crate::svc::cycle::{CycleOptions, CycleOptionsSim};

pub(super) fn get_dps_cycle_options(reload: bool) -> CycleOptions {
    match reload {
        true => CycleOptions::Sim(CycleOptionsSim {
            rearm_minions: Some(true),
            ..
        }),
        false => CycleOptions::Burst,
    }
}

pub(super) const VOLLEY_CYCLE_OPTIONS: CycleOptions = CycleOptions::Burst;
