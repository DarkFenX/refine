use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectHc, eff::shared::opc_rep::get_local_shield_rep_opc},
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIELD_BOOSTING;
const A_EFFECT_ID: AEffectId = ac::effects::SHIELD_BOOSTING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            local_shield_rep_opc_getter: Some(get_local_shield_rep_opc),
            ..
        },
        ..
    }
}
