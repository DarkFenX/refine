use crate::{def::AttrVal, rd::REffectKey};

#[derive(Copy, Clone)]
pub(in crate::svc) enum CycleOptions {
    Burst,
    Sim(CycleOptionsSim),
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptionsSim {
    // Controls if effects which can run with/without charges (e.g. ancillary reps) are forced to
    // reload once they run out of charges
    pub(in crate::svc) reload_optionals: Option<bool> = None,
    // Controls if depleted fighter abilities force fighter recall, refuel and rearm
    pub(in crate::svc) rearm_minions: Option<bool> = None,
}

pub(super) struct SelfKillerInfo {
    pub(super) effect_key: REffectKey,
    pub(super) duration_s: AttrVal,
}
