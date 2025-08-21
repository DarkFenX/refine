use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_noapp_simple_proj_mult, get_simple_mod_proj_attrs},
    },
};

const E_EFFECT_ID: EEffectId = 10_000_000;
const A_EFFECT_ID: AEffectId = AEffectId::Dogma(E_EFFECT_ID);

pub(in crate::nd::eff) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_simple_proj_mult),
            ..
        },
        ..
    }
}
