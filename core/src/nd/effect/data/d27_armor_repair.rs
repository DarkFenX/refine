use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectLocalOpcSpec, NGeneralOutputGetter},
};

const EFFECT_EID: EEffectId = EEffectId::ARMOR_REPAIR;
const EFFECT_AID: AEffectId = AEffectId::ARMOR_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        local_armor_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: NGeneralOutputGetter::RepArmor,
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
