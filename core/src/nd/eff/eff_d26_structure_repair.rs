use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectHc, eff::shared::rep_opc::get_local_hull_rep_opc},
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCTURE_REPAIR;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCTURE_REPAIR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            local_hull_rep_opc_getter: Some(get_local_hull_rep_opc),
            ..
        },
        ..
    }
}
