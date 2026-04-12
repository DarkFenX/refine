use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::FTR_ABIL_ATTACK_M;
const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_ATTACK_M;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::FtrAbilAttackM,
            proj_mult_str: Some(NEffectProjGetter::FtrAbilAttackM),
            ..
        }),
        ..
    }
}
