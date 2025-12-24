use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::get_armor_rep_base_opc, ilimit::get_proj_armor_ilimit, proj_mult::get_simple_s2s_noapp_proj_mult,
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::NPC_ENTITY_REMOTE_ARMOR_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::NPC_ENTITY_REMOTE_ARMOR_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        outgoing_armor_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_armor_rep_base_opc,
            proj_mult: get_simple_s2s_noapp_proj_mult,
            instance_limit: Some(get_proj_armor_ilimit),
            ..
        }),
        ..
    }
}
