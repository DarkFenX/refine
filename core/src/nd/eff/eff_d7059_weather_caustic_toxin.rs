use crate::{ac, ad, ec, nd::NEffect};

pub(super) fn mk_n_effect() -> NEffect {
    // Abyssal exotic weather
    NEffect {
        eid: Some(ec::effects::WEATHER_CAUSTIC_TOXIN),
        aid: ac::effects::WEATHER_CAUSTIC_TOXIN,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
