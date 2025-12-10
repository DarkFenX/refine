use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, effect::data::shared::opc::get_local_armor_rep_opc},
};

const E_EFFECT_ID: EEffectId = ec::effects::ARMOR_REPAIR;
const A_EFFECT_ID: AEffectId = ac::effects::ARMOR_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        local_armor_rep_opc_getter: Some(get_local_armor_rep_opc),
        ..
    }
}
