use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectLocalOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::SHIELD_BOOSTING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        local_shield_rep: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::RepShield,
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
