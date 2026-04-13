use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectGeneralOutputGetter, NEffectLocalOpcSpec,
    },
};

const EFFECT_AID: AEffectId = AEffectId::POWER_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        cap_inject: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::PowerBooster,
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
