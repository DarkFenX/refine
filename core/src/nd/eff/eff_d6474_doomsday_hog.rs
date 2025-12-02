use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{mods::make_dd_self_debuffs, opc::get_aoe_dd_side_neut_opc},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_HOG;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_HOG;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            custom: make_dd_self_debuffs().collect(),
            ..
        }),
        hc: NEffectHc {
            neut_opc_getter: Some(get_aoe_dd_side_neut_opc),
            ..
        },
        ..
    }
}
