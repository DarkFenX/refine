use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEcmOutputGetter, NEffect, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::ECM_BURST_JAMMER;
const EFFECT_AID: AEffectId = AEffectId::ECM_BURST_JAMMER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: NEcmOutputGetter::Burst,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}
