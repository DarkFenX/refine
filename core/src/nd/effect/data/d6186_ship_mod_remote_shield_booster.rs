use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{base_opc::get_shield_rep_base_opc, proj_mult::get_full_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_REMOTE_SHIELD_BOOSTER;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_SHIELD_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        outgoing_shield_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_shield_rep_base_opc,
            proj_mult_str: Some(get_full_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
