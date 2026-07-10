use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectLocalOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::ARMOR_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        local_armor_rep: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::RepArmor,
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
