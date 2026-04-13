use super::shared::{get_aoe_dd_warmup_neut, make_dd_self_debuffs};
use crate::{
    ad::{AEffectBuff, AEffectId},
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_SLASH;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg: Some(NEffectProjOpcSpec {
            // Unlike other AoE doomsdays, reapers hit every ship only once, despite having damage
            // ticks spread over time. We also assume target is hit by first damage tick.
            base: NEffectDmgOutputGetter::Delay2,
            proj_mult_str: Some(NEffectProjGetter::AoeDd),
            ..
        }),
        neut: Some(get_aoe_dd_warmup_neut()),
        ..
    }
}
