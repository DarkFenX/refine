use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplCrystal, NEffectChargeLoc,
        NEffectMiningOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec,
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
        mining_ore_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectMiningOutputGetter::MiningLaserOre,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        mining_ice_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectMiningOutputGetter::MiningLaserIce,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}
