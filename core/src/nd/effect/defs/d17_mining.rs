use crate::{
    ad::AEffectId,
    nd::{
        NEffect, NEffectMining, NEffectMiningChecker, NEffectMiningOutputGetter, NEffectProjGetter, NEffectProjOpcSpec,
    },
};

const EFFECT_AID: AEffectId = AEffectId::MINING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        mining_ore: Some(NEffectMining {
            checker: Some(NEffectMiningChecker::NonIce),
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Regular,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        mining_ice: Some(NEffectMining {
            checker: Some(NEffectMiningChecker::Ice),
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Regular,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        ..
    }
}
