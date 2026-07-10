use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectProjecteeFilter,
    },
};

const EFFECT_AID: AEffectId = AEffectId::USE_MISSILES;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: true,
        }),
        projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(AAttrId::VALID_TGT_WHITELIST)),
        ..
    }
}
