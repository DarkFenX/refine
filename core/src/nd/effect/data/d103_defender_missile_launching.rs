use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_instant_dmg_base_opc, proj_mult::get_null_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DEFENDER_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::DEFENDER_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            // Defenders cannot be used vs targets allowed by the lib, so always return 0 if target
            // is specified
            proj_mult_pre: Some(get_null_proj_mult),
            ..
        }),
        ..
    }
}
