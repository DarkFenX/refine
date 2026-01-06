use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectLocalOpcSpec, effect::data::shared::base_opc::get_armor_rep_base_opc},
};

const EFFECT_EID: EEffectId = EEffectId::ARMOR_REPAIR;
const EFFECT_AID: AEffectId = AEffectId::ARMOR_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        local_armor_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_armor_rep_base_opc,
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
