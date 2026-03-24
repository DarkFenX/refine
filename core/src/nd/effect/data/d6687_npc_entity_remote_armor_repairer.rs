use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjMultGetterX, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::base_opc::get_armor_rep_base_opc,
    },
};

const EFFECT_EID: EEffectId = EEffectId::NPC_ENTITY_REMOTE_ARMOR_REPAIRER;
const EFFECT_AID: AEffectId = AEffectId::NPC_ENTITY_REMOTE_ARMOR_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        outgoing_armor_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_armor_rep_base_opc,
            proj_mult_str: Some(NEffectProjMultGetterX::RangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
