use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectEcmOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_ECM;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectEcmOutputGetter::Aoe,
            proj_mult_str: Some(NEffectProjMultGetter::AoeBurstRange),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}
