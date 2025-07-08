use crate::{ac, ad, ec, nd::NEffect};

pub(super) fn mk_n_effect() -> NEffect {
    // Abyssal gamma weather
    NEffect {
        eid: Some(ec::effects::WEATHER_XENON_GAS),
        aid: ac::effects::WEATHER_XENON_GAS,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
