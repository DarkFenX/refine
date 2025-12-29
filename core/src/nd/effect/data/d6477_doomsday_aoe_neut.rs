use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{base_opc::get_aoe_neut_base_opc, proj_mult::get_aoe_burst_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_NEUT;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_NEUT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: get_aoe_neut_base_opc,
            proj_mult_str: Some(get_aoe_burst_proj_mult),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
