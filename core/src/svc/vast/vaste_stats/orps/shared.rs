use crate::svc::cycle::{CycleOptions, CycleOptionsSim};

pub(super) fn get_orps_cycle_options(reload: bool) -> CycleOptions {
    match reload {
        true => CycleOptions::Sim(CycleOptionsSim {
            reload_optionals: Some(true),
            ..
        }),
        false => CycleOptions::Burst,
    }
}
