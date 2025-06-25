use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::FTR_ABIL_BOMB),
        aid: ac::effects::FTR_ABIL_BOMB,
        charge_info: Some(ad::AEffectChargeInfo::Attr(ac::attrs::FTR_ABIL_BOMB_TYPE)),
        ..
    }
}
