use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::get_cap_trans_base_opc, ilimit::get_proj_cap_ilimit, proj_mult::get_simple_s2s_noapp_proj_mult,
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        outgoing_cap_opc_spec: Some(NEffectProjOpcSpec {
            base: get_cap_trans_base_opc,
            proj_mult: get_simple_s2s_noapp_proj_mult,
            instance_limit: Some(get_proj_cap_ilimit),
            ..
        }),
        ..
    }
}
