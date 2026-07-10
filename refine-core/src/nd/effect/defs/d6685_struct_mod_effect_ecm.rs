use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectEcm, NEffectEcmOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_AID: AEffectId = AEffectId::STRUCT_MOD_EFFECT_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        ecm: Some(NEffectEcm {
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Direct,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}
