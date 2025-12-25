use crate::{
    ac,
    ad::{AEffectBuff, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::{base_opc::get_aoe_dd_side_neut_opc_spec, mods::make_dd_self_debuffs},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_HOG;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_HOG;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        neut_opc_spec: Some(get_aoe_dd_side_neut_opc_spec()),
        ..
    }
}
