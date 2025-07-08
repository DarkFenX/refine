use crate::{ac, ad, ec, nd::NEffect};

pub(super) fn mk_n_effect() -> NEffect {
    // Abyssal electrical weather
    NEffect {
        eid: Some(ec::effects::WEATHER_ELECTRIC_STORM),
        aid: ac::effects::WEATHER_ELECTRIC_STORM,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
