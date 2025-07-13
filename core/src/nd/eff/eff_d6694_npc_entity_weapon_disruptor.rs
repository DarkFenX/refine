use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
            wd::update_effect_td,
        },
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::NPC_ENTITY_WEAPON_DISRUPTOR;
const A_EFFECT_ID: ad::AEffectId = ac::effects::NPC_ENTITY_WEAPON_DISRUPTOR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect_td(A_EFFECT_ID, a_effect)),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
