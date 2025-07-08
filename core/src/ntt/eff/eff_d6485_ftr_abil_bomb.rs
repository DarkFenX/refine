use crate::{
    ac, ec,
    ntt::{NttEffect, NttEffectCharge, NttEffectHc},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::FTR_ABIL_BOMB),
        aid: ac::effects::FTR_ABIL_BOMB,
        hc: NttEffectHc {
            charge: Some(NttEffectCharge::Attr(ac::attrs::FTR_ABIL_BOMB_TYPE)),
            ..
        },
        ..
    }
}
