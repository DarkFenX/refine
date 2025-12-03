use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{
            mods::make_dd_self_debuffs,
            opc::{get_aoe_dd_dmg_opc, get_aoe_dd_side_neut_opc},
        },
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_CONE_DOT;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_CONE_DOT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(get_aoe_dd_dmg_opc),
            neut_opc_getter: Some(get_aoe_dd_side_neut_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
