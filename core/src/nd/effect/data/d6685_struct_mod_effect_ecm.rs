use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{base_opc::get_direct_ecm_base_opc, proj_mult::get_full_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCT_MOD_EFFECT_ECM;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCT_MOD_EFFECT_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: get_direct_ecm_base_opc,
            proj_mult_str: Some(get_full_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}
