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

const EFFECT_EID: EEffectId = EEffectId::STRUCT_MOD_EFFECT_ECM;
const EFFECT_AID: AEffectId = AEffectId::STRUCT_MOD_EFFECT_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: get_direct_ecm_base_opc,
            proj_mult_str: Some(get_full_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}
