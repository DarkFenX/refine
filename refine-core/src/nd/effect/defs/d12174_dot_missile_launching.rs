use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectBreacherOutputGetter, NEffectDmgKindGetter, NEffectProjGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_AID: AEffectId = AEffectId::DOT_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Breacher),
        breacher_dmg: Some(NEffectProjOpcSpec {
            base: NEffectBreacherOutputGetter::Regular,
            proj_mult_chance: Some(NEffectProjGetter::MissileRange),
            resist: Some(NEffectResist::Attr(AAttrId::BREACHER_POD_DMG_RESIST)),
            ..
        }),
        ..
    }
}
