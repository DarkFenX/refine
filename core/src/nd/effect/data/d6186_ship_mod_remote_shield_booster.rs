use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist, NGeneralOutputGetter},
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_REMOTE_SHIELD_BOOSTER;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_SHIELD_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        outgoing_shield_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: NGeneralOutputGetter::RepShield,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeFullStsRestricted),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
