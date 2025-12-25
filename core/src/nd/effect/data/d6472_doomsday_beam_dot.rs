use crate::{
    ac,
    ad::{AEffectBuff, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind,
        effect::data::shared::{
            base_opc::{get_aoe_dd_dmg_opc, get_aoe_dd_side_neut_opc_spec},
            mods::make_dd_self_debuffs,
        },
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_BEAM_DOT;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_BEAM_DOT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_getter: Some(get_aoe_dd_dmg_opc),
        neut_opc_spec: Some(get_aoe_dd_side_neut_opc_spec()),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
