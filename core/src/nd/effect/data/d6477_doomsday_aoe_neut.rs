use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::base_opc::get_aoe_neut_base_opc,
    },
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_NEUT;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_NEUT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: get_aoe_neut_base_opc,
            proj_mult_str: Some(NEffectProjMultGetter::AoeBurst),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
