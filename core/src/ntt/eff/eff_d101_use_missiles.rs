use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::USE_MISSILES),
        aid: ac::effects::USE_MISSILES,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        ..
    }
}
