use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal red cloud
pub(crate) const EFF_D7051: NttEffect = NttEffect {
    eid: Some(ec::effects::AOE_BEACON_CAUSTIC_CLOUD),
    aid: ac::effects::AOE_BEACON_CAUSTIC_CLOUD,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
