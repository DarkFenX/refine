use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    // Abyssal exotic weather
    NttEffect {
        eid: Some(ec::effects::WEATHER_CAUSTIC_TOXIN),
        aid: ac::effects::WEATHER_CAUSTIC_TOXIN,
        buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
