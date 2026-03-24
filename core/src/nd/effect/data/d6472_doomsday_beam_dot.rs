use crate::{
    ad::{AEffectBuff, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKindGetter,
        effect::data::shared::{
            base_opc::{get_aoe_dd_dmg_opc_spec, get_aoe_dd_side_neut_opc_spec},
            mods::make_dd_self_debuffs,
        },
    },
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_BEAM_DOT;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_BEAM_DOT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind_getter: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg_opc_spec: Some(get_aoe_dd_dmg_opc_spec()),
        neut_opc_spec: Some(get_aoe_dd_side_neut_opc_spec()),
        ..
    }
}
