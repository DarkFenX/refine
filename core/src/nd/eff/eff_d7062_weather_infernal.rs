use crate::{ac, ad, ec, nd::NEffect};

pub(super) fn mk_n_effect() -> NEffect {
    // Abyssal firestorm weather
    NEffect {
        eid: Some(ec::effects::WEATHER_INFERNAL),
        aid: ac::effects::WEATHER_INFERNAL,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
