use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectChargeMultGetter, NEffectGeneralOutputGetter, NEffectLocalOpcSpec,
    },
};

const EFFECT_AID: AEffectId = AEffectId::FUELED_SHIELD_BOOSTING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate {
                can_run_uncharged: true,
            })),
            activates_charge: false,
        }),
        local_shield_rep: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::RepShield,
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        cap_consume: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::CapConsumer,
            charge_mult: Some(NEffectChargeMultGetter::AsbCap),
            ..
        }),
        ..
    }
}
