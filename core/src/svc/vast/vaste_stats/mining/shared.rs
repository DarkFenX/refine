use crate::svc::cycle::{CycleOptionsSim, CyclingOptions};

pub(super) fn get_mps_cycle_options(reload: bool) -> CyclingOptions {
    match reload {
        true => CyclingOptions::Sim(CycleOptionsSim {
            reload_optionals: Some(true),
            ..
        }),
        false => CyclingOptions::Burst,
    }
}
