use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal gamma weather
pub(crate) const EFF_D7063: NttEffect = NttEffect {
    eid: Some(ec::effects::WEATHER_XENON_GAS),
    aid: ac::effects::WEATHER_XENON_GAS,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
