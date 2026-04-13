use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectLocalOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::STRUCTURE_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        local_hull_rep: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::RepHull,
            limit_attr_id: Some(AAttrId::HP),
            ..
        }),
        ..
    }
}
