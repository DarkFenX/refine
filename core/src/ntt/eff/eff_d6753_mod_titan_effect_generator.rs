use crate::{
    ac, ad, ec,
    ntt::{
        NttEffect, NttEffectRt,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::MOD_TITAN_EFFECT_GENERATOR),
        aid: ac::effects::MOD_TITAN_EFFECT_GENERATOR,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::Ships,
        }),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        rt: NttEffectRt {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
