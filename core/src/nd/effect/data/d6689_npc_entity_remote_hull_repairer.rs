use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::NPC_ENTITY_REMOTE_HULL_REPAIRER;
const EFFECT_AID: AEffectId = AEffectId::NPC_ENTITY_REMOTE_HULL_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        outgoing_hull_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::RepHull,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::HP),
            ..
        }),
        ..
    }
}
