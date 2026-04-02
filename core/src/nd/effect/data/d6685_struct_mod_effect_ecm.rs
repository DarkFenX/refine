use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectEcm, NEffectEcmOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::STRUCT_MOD_EFFECT_ECM;
const EFFECT_AID: AEffectId = AEffectId::STRUCT_MOD_EFFECT_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        ecm: Some(NEffectEcm {
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Direct,
                proj_mult_str: Some(NEffectProjMultGetter::GenericRangeFullStsRestricted),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}
