use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::SHIP_MODULE_RASB),
        aid: ac::effects::SHIP_MODULE_RASB,
        charge_info: Some(ad::AEffectChargeInfo::Loaded),
        ..
    }
}
