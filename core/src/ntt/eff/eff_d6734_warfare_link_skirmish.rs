use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH),
        aid: ac::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH,
        buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::FleetShips,
        }),
        charge_info: Some(ad::AEffectChargeInfo::Loaded),
        ..
    }
}
