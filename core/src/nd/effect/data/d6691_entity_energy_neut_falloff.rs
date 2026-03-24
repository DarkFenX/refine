use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjMultGetterX, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::base_opc::get_neut_base_opc,
    },
};

const EFFECT_EID: EEffectId = EEffectId::ENTITY_ENERGY_NEUT_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::ENTITY_ENERGY_NEUT_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: get_neut_base_opc,
            proj_mult_str: Some(NEffectProjMultGetterX::RangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
