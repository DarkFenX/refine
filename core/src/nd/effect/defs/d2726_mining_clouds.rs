use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectMining, NEffectMiningOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::MINING_CLOUDS;
const EFFECT_AID: AEffectId = AEffectId::MINING_CLOUDS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        mining_gas: Some(NEffectMining {
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Regular,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        ..
    }
}
