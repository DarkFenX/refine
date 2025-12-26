use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_instant_dmg_base_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::EMP_WAVE;
const A_EFFECT_ID: AEffectId = ac::effects::EMP_WAVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Smartbomb
}
