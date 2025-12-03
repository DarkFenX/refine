use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectHc, effect::shared::opc::get_local_armor_rep_opc},
};

const E_EFFECT_ID: EEffectId = ec::effects::ARMOR_REPAIR;
const A_EFFECT_ID: AEffectId = ac::effects::ARMOR_REPAIR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            local_armor_rep_opc_getter: Some(get_local_armor_rep_opc),
            ..
        },
        ..
    }
}
