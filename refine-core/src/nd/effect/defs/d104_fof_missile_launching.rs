use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::FOF_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Missile),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::MissileApplication),
            proj_mult_chance: Some(NEffectProjMultGetter::MissileRangeFof),
            ..
        }),
        ..
    }
}
