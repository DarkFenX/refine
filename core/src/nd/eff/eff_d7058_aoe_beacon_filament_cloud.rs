use crate::{ac, ad, ec, nd::NEffect};

pub(super) fn mk_n_effect() -> NEffect {
    // Abyssal white cloud
    NEffect {
        eid: Some(ec::effects::AOE_BEACON_FILAMENT_CLOUD),
        aid: ac::effects::AOE_BEACON_FILAMENT_CLOUD,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
