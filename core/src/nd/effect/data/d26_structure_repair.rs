use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectLocalOpcSpec, effect::data::shared::base_opc::get_hull_rep_base_opc},
};

const EFFECT_EID: EEffectId = EEffectId::STRUCTURE_REPAIR;
const EFFECT_AID: AEffectId = AEffectId::STRUCTURE_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        local_hull_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_hull_rep_base_opc,
            limit_attr_id: Some(AAttrId::HP),
            ..
        }),
        ..
    }
}
