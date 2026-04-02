use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectMining, NEffectMiningChecker, NEffectMiningOutputGetter, NEffectProjMultGetter,
        NEffectProjOpcSpec,
    },
};

const EFFECT_EID: EEffectId = EEffectId::MINING;
const EFFECT_AID: AEffectId = AEffectId::MINING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        mining_ore: Some(NEffectMining {
            checker: Some(NEffectMiningChecker::NonIce),
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Regular,
                proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        mining_ice: Some(NEffectMining {
            checker: Some(NEffectMiningChecker::Ice),
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Regular,
                proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        ..
    }
}
