use crate::{
    misc::{PValue, RearmMinions, ReloadOptionals, StOption},
    rd::REffectId,
    svc::vast::{StatTimeOptions, StatTimeOptionsSim},
};

#[derive(Copy, Clone)]
pub(in crate::svc) enum CyclingOptions {
    Burst,
    Sim(CycleOptionsSim),
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptionsSim {
    // Controls if effects which can run with/without charges (e.g. ancillary reps) are forced to
    // reload once they run out of charges
    pub(in crate::svc) reload_optionals: StOption<ReloadOptionals> = StOption::Inherit,
    // Controls if depleted fighter abilities force fighter recall, refuel and rearm
    pub(in crate::svc) rearm_minions: StOption<RearmMinions> = StOption::Inherit,
}

pub(super) struct SelfKillerInfo {
    pub(super) effect_rid: REffectId,
    pub(super) duration: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CyclingOptions {
    pub(in crate::svc::cycle) fn from_time_options(time_options: StatTimeOptions) -> Self {
        match time_options {
            StatTimeOptions::Burst(_) => Self::Burst,
            StatTimeOptions::Sim(inner) => Self::Sim(CycleOptionsSim::from_time_options_sim(inner)),
        }
    }
}

impl CycleOptionsSim {
    fn from_time_options_sim(time_options_sim: StatTimeOptionsSim) -> Self {
        Self {
            reload_optionals: time_options_sim.reload_optionals,
            rearm_minions: time_options_sim.rearm_minions,
        }
    }
}
