use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjMultGetterX, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::base_opc::get_ecm_drone_base_opc,
    },
};

const EFFECT_EID: EEffectId = EEffectId::ENTITY_ECM_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::ENTITY_ECM_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: get_ecm_drone_base_opc,
            proj_mult_str: Some(NEffectProjMultGetterX::RangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}
