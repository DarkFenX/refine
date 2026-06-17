use crate::{
    ad::AEffectId,
    nd::{
        NEffect, NEffectDuration, NEffectEcm, NEffectEcmOutputGetter, NEffectProjGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        disallows_cloak: Some(NEffectDuration::Effect),
        ecm: Some(NEffectEcm {
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Aoe,
                proj_mult_str: Some(NEffectProjGetter::AoeBurstRange),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}
