use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::SHIP_MODULE_RAAR),
        aid: ac::effects::SHIP_MODULE_RAAR,
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        ..
    }
}
