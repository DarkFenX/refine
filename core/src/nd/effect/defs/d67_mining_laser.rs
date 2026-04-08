use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplCrystal, NEffectChargeLoc, NEffectMining,
        NEffectMiningChecker, NEffectMiningOutputGetter, NEffectProjGetter, NEffectProjOpcSpec,
    },
};

const EFFECT_EID: EEffectId = EEffectId::MINING_LASER;
const EFFECT_AID: AEffectId = AEffectId::MINING_LASER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Crystal(NEffectChargeDeplCrystal {
                can_run_uncharged: true,
            })),
            activates_charge: false,
        }),
        mining_ore: Some(NEffectMining {
            checker: Some(NEffectMiningChecker::NonIce),
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Crit,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        mining_ice: Some(NEffectMining {
            checker: Some(NEffectMiningChecker::Ice),
            ospec: NEffectProjOpcSpec {
                base: NEffectMiningOutputGetter::Crit,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
                ..
            },
        }),
        ..
    }
}
