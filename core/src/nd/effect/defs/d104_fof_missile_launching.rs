use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::FOF_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::FOF_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Missile),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjGetter::MissileApplication),
            proj_mult_chance: Some(NEffectProjGetter::MissileRangeFof),
            ..
        }),
        ..
    }
}
