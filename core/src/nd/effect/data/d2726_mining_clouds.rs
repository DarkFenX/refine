use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_mining_base_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING_CLOUDS;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_CLOUDS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        mining_gas_opc_spec: Some(NEffectProjOpcSpec {
            base: get_mining_base_opc,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            ..
        }),
        ..
    }
}
