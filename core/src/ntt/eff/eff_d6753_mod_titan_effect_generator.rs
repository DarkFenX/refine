use crate::{ac, ad, ec, ntt::NttEffect};

pub(crate) const EFF_D6753: NttEffect = NttEffect {
    eid: Some(ec::effects::MOD_TITAN_EFFECT_GENERATOR),
    aid: ac::effects::MOD_TITAN_EFFECT_GENERATOR,
    buff_info: Some(ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::DefaultAttrs,
        scope: ad::AEffectBuffScope::Ships,
    }),
    ..
};
