use crate::{ac, ad, ec, ntt::NttEffect};

pub(crate) const EFF_D6733: NttEffect = NttEffect {
    eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_SHIELD),
    aid: ac::effects::MOD_BONUS_WARFARE_LINK_SHIELD,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::FleetShips,
    }),
    charge_info: Some(ad::AEffectChargeInfo::Loaded),
    ..
};
