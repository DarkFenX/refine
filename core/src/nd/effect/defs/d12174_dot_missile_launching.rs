use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectBreacherOutputGetter, NEffectDmgKindGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::DOT_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::DOT_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Breacher),
        breacher_dmg: Some(NEffectProjOpcSpec {
            base: NEffectBreacherOutputGetter::Regular,
            proj_mult_chance: Some(NEffectProjMultGetter::MissileRange),
            ..
        }),
        ..
    }
}
