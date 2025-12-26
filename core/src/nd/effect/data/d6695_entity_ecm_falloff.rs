use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{base_opc::get_ecm_drone_base_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::ENTITY_ECM_FALLOFF;
const A_EFFECT_ID: AEffectId = ac::effects::ENTITY_ECM_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: get_ecm_drone_base_opc,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}
