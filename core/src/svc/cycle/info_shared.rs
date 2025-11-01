use crate::{def::AttrVal, rd::REffectKey};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptions {
    pub(in crate::svc) reload_mode: CycleOptionReload,
    // Controls if effects which can run with/without charges (e.g. ancillary reps) are forced to
    // reload once they run out of charges
    pub(in crate::svc) reload_optionals: bool,
}

#[derive(Copy, Clone)]
pub(in crate::svc) enum CycleOptionReload {
    // Assumes reload time is 0, so that effects can cycle infinitely (reload is still considered
    // for purposes like spoolup)
    Burst,
    // Respects reload time
    Sim,
}

pub(super) struct SelfKillerInfo {
    pub(super) effect_key: REffectKey,
    pub(super) duration_s: AttrVal,
}
