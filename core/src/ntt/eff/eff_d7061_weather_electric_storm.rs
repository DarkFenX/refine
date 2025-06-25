use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal electrical weather
pub(crate) const EFF_D7061: NttEffect = NttEffect {
    eid: Some(ec::effects::WEATHER_ELECTRIC_STORM),
    aid: ac::effects::WEATHER_ELECTRIC_STORM,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
