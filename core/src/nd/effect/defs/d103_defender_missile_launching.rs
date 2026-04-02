use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectDmgOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::DEFENDER_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::DEFENDER_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::Null),
            ..
        }),
        ..
    }
}
