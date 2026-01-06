use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_instant_dmg_base_opc, proj_mult::get_null_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::DEFENDER_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::DEFENDER_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(get_null_proj_mult),
            ..
        }),
        ..
    }
}
