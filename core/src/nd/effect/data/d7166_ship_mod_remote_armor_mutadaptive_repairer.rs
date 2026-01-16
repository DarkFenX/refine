use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist, NEffectSpoolAttrs,
        effect::data::shared::{base_opc::get_armor_rep_base_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        spool_attrs: Some(NEffectSpoolAttrs {
            step_attr_id: AAttrId::REP_MULT_BONUS_PER_CYCLE,
            max_attr_id: AAttrId::REP_MULT_BONUS_MAX,
        }),
        outgoing_armor_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_armor_rep_base_opc,
            spoolable: true,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
