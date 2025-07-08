use crate::{
    ac, ad, ec,
    ntt::{NttEffect, NttEffectCharge, NttEffectChargeDepl, NttEffectHc},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_SHIELD),
        aid: ac::effects::MOD_BONUS_WARFARE_LINK_SHIELD,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::FleetShips,
        }),
        hc: NttEffectHc {
            charge: Some(NttEffectCharge::Loaded(NttEffectChargeDepl::ChargeRate)),
            ..
        },
        ..
    }
}
