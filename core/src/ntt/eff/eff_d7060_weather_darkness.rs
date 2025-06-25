use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal dark weather
pub(crate) const EFF_D7060: NttEffect = NttEffect {
    eid: Some(ec::effects::WEATHER_DARKNESS),
    aid: ac::effects::WEATHER_DARKNESS,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
