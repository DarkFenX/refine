use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectEcm, NEffectEcmOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_ECM_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_ECM_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
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
