use crate::{
    ac, ec,
    ntt::{NttEffect, NttEffectCharge, NttEffectChargeDepl, NttEffectHc},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::USE_MISSILES),
        aid: ac::effects::USE_MISSILES,
        hc: NttEffectHc {
            charge: Some(NttEffectCharge::Loaded(NttEffectChargeDepl::ChargeRate)),
            ..
        },
        ..
    }
}
