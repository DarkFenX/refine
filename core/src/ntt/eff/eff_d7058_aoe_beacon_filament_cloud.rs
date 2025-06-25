use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal white cloud
pub(crate) const EFF_D7058: NttEffect = NttEffect {
    eid: Some(ec::effects::AOE_BEACON_FILAMENT_CLOUD),
    aid: ac::effects::AOE_BEACON_FILAMENT_CLOUD,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
