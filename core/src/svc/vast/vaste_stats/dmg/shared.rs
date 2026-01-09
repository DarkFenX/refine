use crate::svc::cycle::{CycleOptionsSim, CyclingOptions};

pub(super) fn get_dps_cycling_options(reload: bool) -> CyclingOptions {
    match reload {
        true => CyclingOptions::Sim(CycleOptionsSim {
            rearm_minions: Some(true),
            ..
        }),
        false => CyclingOptions::Burst,
    }
}

pub(super) const VOLLEY_CYCLE_OPTIONS: CyclingOptions = CyclingOptions::Burst;
