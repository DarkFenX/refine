use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    // Abyssal electrical weather
    NttEffect {
        eid: Some(ec::effects::WEATHER_ELECTRIC_STORM),
        aid: ac::effects::WEATHER_ELECTRIC_STORM,
        buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
