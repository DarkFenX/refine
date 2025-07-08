use crate::{ac, ad, ec, nd::NEffect};

pub(super) fn mk_n_effect() -> NEffect {
    // Abyssal tracking towers
    NEffect {
        eid: Some(ec::effects::AOE_BEACON_PULSE_01),
        aid: ac::effects::AOE_BEACON_PULSE_01,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
