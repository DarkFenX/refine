use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{
            dmg_opc::get_dmg_opc_missile,
            proj_mult::{get_proj_attrs_missile, get_proj_mult_missile},
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_missile),
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Missile),
            proj_mult_getter: Some(get_proj_mult_missile),
            normal_dmg_opc_getter: Some(get_dmg_opc_missile),
            ..
        },
        ..
    }
}
