use crate::{ad, def::AttrVal};

#[derive(Copy, Clone)]
pub(in crate::svc) enum CycleOptions {
    Burst,
    Sim,
}

pub(super) struct SelfKillerInfo {
    pub(super) a_effect_id: ad::AEffectId,
    pub(super) duration_s: AttrVal,
}
