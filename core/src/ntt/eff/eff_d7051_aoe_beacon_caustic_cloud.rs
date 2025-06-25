use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    // Abyssal red cloud
    NttEffect {
        eid: Some(ec::effects::AOE_BEACON_CAUSTIC_CLOUD),
        aid: ac::effects::AOE_BEACON_CAUSTIC_CLOUD,
        buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}
