use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{base_opc::get_neut_base_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::ENTITY_ENERGY_NEUT_FALLOFF;
const A_EFFECT_ID: AEffectId = ac::effects::ENTITY_ENERGY_NEUT_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: get_neut_base_opc,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            ilimit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
