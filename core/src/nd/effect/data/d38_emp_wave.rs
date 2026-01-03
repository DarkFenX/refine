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

const EFFECT_EID: EEffectId = ec::effects::EMP_WAVE;
const EFFECT_AID: AEffectId = ac::effects::EMP_WAVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
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
