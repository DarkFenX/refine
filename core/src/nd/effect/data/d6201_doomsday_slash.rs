use crate::{
    ad::{AEffectBuff, AEffectId},
    ed::EEffectId,
    nd::{
        NDmgOutputGetter, NEffect, NEffectDmgKindGetter, NEffectProjMultGetter, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_aoe_dd_side_neut_opc_spec, mods::make_dd_self_debuffs},
    },
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_SLASH;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_SLASH;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind_getter: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            // Unlike other AoE doomsdays, reapers hit every ship only once, despite having damage
            // ticks spread over time. We also assume target is hit by first damage tick.
            base: NDmgOutputGetter::Delay2,
            proj_mult_str: Some(NEffectProjMultGetter::AoeDd),
            ..
        }),
        neut_opc_spec: Some(get_aoe_dd_side_neut_opc_spec()),
        ..
    }
}
