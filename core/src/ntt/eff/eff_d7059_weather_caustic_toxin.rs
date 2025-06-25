use crate::{ac, ad, ec, ntt::NttEffect};

// Abyssal exotic weather
pub(crate) const EFF_D7059: NttEffect = NttEffect {
    eid: Some(ec::effects::WEATHER_CAUSTIC_TOXIN),
    aid: ac::effects::WEATHER_CAUSTIC_TOXIN,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Everything,
    }),
    ..
};
