use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::NPC_ENTITY_REMOTE_ARMOR_REPAIRER;
const EFFECT_AID: AEffectId = AEffectId::NPC_ENTITY_REMOTE_ARMOR_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        outgoing_armor_rep: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::RepArmor,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            remote_limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
