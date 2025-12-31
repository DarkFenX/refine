use crate::{
    def::AttrVal,
    rd::REffectKey,
    svc::vast::{StatTimeOptions, StatTimeOptionsSim},
};

#[derive(Copy, Clone)]
pub(in crate::svc) enum CyclingOptions {
    Burst,
    Sim(CycleOptionsSim),
}
impl From<StatTimeOptions> for CyclingOptions {
    fn from(time_options: StatTimeOptions) -> Self {
        match time_options {
            StatTimeOptions::Burst(_) => Self::Burst,
            StatTimeOptions::Sim(inner) => Self::Sim(inner.into()),
        }
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptionsSim {
    // Controls if effects which can run with/without charges (e.g. ancillary reps) are forced to
    // reload once they run out of charges
    pub(in crate::svc) reload_optionals: Option<bool> = None,
    // Controls if depleted fighter abilities force fighter recall, refuel and rearm
    pub(in crate::svc) rearm_minions: Option<bool> = None,
}
impl From<StatTimeOptionsSim> for CycleOptionsSim {
    fn from(time_options: StatTimeOptionsSim) -> Self {
        Self {
            reload_optionals: time_options.reload_optionals,
            rearm_minions: time_options.rearm_minions,
        }
    }
}

pub(super) struct SelfKillerInfo {
    pub(super) effect_key: REffectKey,
    pub(super) duration: AttrVal,
}
