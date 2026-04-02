use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectLocalOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::STRUCTURE_REPAIR;
const EFFECT_AID: AEffectId = AEffectId::STRUCTURE_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        local_hull_rep: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::RepHull,
            limit_attr_id: Some(AAttrId::HP),
            ..
        }),
        ..
    }
}
