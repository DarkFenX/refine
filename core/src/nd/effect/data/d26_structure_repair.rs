use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectLocalOpcSpec,
        effect::data::shared::opc::{get_local_hull_rep_base_opc, get_local_hull_rep_ilimit, get_local_hull_rep_opc},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCTURE_REPAIR;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCTURE_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        local_hull_rep_output: Some(NEffectLocalOpcSpec {
            base: get_local_hull_rep_base_opc,
            instance_limit: Some(get_local_hull_rep_ilimit),
            ..
        }),
        local_hull_rep_opc_getter: Some(get_local_hull_rep_opc),
        ..
    }
}
