use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_MISSILES;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::FtrAbilMissiles,
            proj_mult_str: Some(NEffectProjGetter::FtrAbilMissiles),
            resist: Some(NEffectResist::AttrRef(AAttrId::FTR_ABIL_MISSILES_RESIST_ID)),
            ..
        }),
        ..
    }
}
