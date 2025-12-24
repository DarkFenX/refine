use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectLocalOpcSpec,
        effect::data::shared::{base_opc::get_hull_rep_base_opc, ilimit::get_self_hull_ilimit},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCTURE_REPAIR;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCTURE_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        local_hull_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_hull_rep_base_opc,
            instance_limit: Some(get_self_hull_ilimit),
            ..
        }),
        ..
    }
}
