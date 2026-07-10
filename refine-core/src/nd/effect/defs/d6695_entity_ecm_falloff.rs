use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectEcm, NEffectEcmOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_AID: AEffectId = AEffectId::ENTITY_ECM_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        ecm: Some(NEffectEcm {
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Entity,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}
