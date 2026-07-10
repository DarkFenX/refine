use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist, NEffectTime},
    num::PValue,
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_KAMIKAZE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        kills_item: Some(NEffectTime::Hardcoded(PValue::ZERO)),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::FtrAbilKamikaze,
            proj_mult_str: Some(NEffectProjGetter::FtrAbilKamikaze),
            resist: Some(NEffectResist::AttrRef(AAttrId::FTR_ABIL_KAMIKAZE_RESIST_ID)),
            ..
        }),
        ..
    }
}
