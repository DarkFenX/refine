use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjMultGetter, NEffectProjOpcSpec, NMiningOutputGetter},
};

const EFFECT_EID: EEffectId = EEffectId::MINING_CLOUDS;
const EFFECT_AID: AEffectId = AEffectId::MINING_CLOUDS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        mining_gas_opc_spec: Some(NEffectProjOpcSpec {
            base: NMiningOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}
