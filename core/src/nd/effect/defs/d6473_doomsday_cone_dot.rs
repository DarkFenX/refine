use super::shared::{get_aoe_dd_warmup_neut, make_dd_self_debuffs};
use crate::{
    ad::{AEffectBuff, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_CONE_DOT;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_CONE_DOT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind: Some(NEffectDmgKindGetter::Superweapon),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::DotDelay,
            proj_mult_str: Some(NEffectProjMultGetter::AoeDd),
            ..
        }),
        neut: Some(get_aoe_dd_warmup_neut()),
        ..
    }
}
