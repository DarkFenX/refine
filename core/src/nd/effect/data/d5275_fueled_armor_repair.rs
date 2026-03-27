use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectChargeMultGetter, NEffectGeneralOutputGetter, NEffectLocalOpcSpec,
    },
};

const EFFECT_EID: EEffectId = EEffectId::FUELED_ARMOR_REPAIR;
const EFFECT_AID: AEffectId = AEffectId::FUELED_ARMOR_REPAIR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate {
                can_run_uncharged: true,
            })),
            activates_charge: false,
        }),
        local_armor_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: NEffectGeneralOutputGetter::RepArmor,
            charge_mult: Some(NEffectChargeMultGetter::AarRep),
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
