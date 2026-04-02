use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectProjecteeFilter,
    },
};

const EFFECT_EID: EEffectId = EEffectId::USE_MISSILES;
const EFFECT_AID: AEffectId = AEffectId::USE_MISSILES;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: true,
        }),
        projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(AAttrId::VALID_TGT_WHITELIST)),
        ..
    }
}
