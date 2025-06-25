use crate::{ac, ad, ec, ntt::NttEffect};

const EFF_6732: NttEffect = NttEffect {
    eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_ARMOR),
    aid: ac::effects::MOD_BONUS_WARFARE_LINK_ARMOR,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::FleetShips,
    }),
    charge_info: Some(ad::AEffectChargeInfo::Loaded),
    ..
};
