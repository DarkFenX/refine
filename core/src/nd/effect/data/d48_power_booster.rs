use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectLocalOpcSpec,
        NGeneralOutputGetter,
    },
};

const EFFECT_EID: EEffectId = EEffectId::POWER_BOOSTER;
const EFFECT_AID: AEffectId = AEffectId::POWER_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        cap_inject_opc_spec: Some(NEffectLocalOpcSpec {
            base: NGeneralOutputGetter::PowerBooster,
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
