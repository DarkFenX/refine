use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectLocalOpcSpec,
        effect::data::shared::opc::{get_local_shield_rep_base_opc, get_local_shield_rep_ilimit},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIELD_BOOSTING;
const A_EFFECT_ID: AEffectId = ac::effects::SHIELD_BOOSTING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        local_shield_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_local_shield_rep_base_opc,
            instance_limit: Some(get_local_shield_rep_ilimit),
            ..
        }),
        ..
    }
}
