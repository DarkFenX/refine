use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectLocalOpcSpec, effect::data::shared::base_opc::get_shield_rep_base_opc},
};

const EFFECT_EID: EEffectId = EEffectId::SHIELD_BOOSTING;
const EFFECT_AID: AEffectId = AEffectId::SHIELD_BOOSTING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        local_shield_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_shield_rep_base_opc,
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
