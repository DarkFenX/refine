use super::shared::{get_aoe_dd_warmup_neut, make_dd_self_debuffs};
use crate::{
    ad::{AEffectBuff, AEffectId},
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_CONE_DOT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::DotDelay,
            proj_mult_str: Some(NEffectProjGetter::AoeDd),
            ..
        }),
        neut: Some(get_aoe_dd_warmup_neut()),
        ..
    }
}
