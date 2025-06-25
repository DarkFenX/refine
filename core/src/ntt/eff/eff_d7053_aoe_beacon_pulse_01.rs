use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal tracking towers
pub(crate) const EFF_D7053: NttEffect = NttEffect {
    eid: Some(ec::effects::AOE_BEACON_PULSE_01),
    aid: ac::effects::AOE_BEACON_PULSE_01,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
