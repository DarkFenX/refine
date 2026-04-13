use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DEFENDER_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjGetter::Null),
            ..
        }),
        ..
    }
}
