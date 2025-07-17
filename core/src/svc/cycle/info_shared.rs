use crate::{ad, def::AttrVal};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptions {
    pub(in crate::svc) reload_mode: CycleOptionReload,
    // Controls if optional reloads are executed, or item keeps running without reloading
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
    pub(super) a_effect_id: ad::AEffectId,
    pub(super) duration_s: AttrVal,
}
