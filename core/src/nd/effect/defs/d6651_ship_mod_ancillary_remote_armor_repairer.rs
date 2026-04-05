use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectChargeMultGetter, NEffectGeneralOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist,
    },
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER;

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
        outgoing_armor_rep: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::RepArmor,
            charge_mult: Some(NEffectChargeMultGetter::AarRep),
            proj_mult_str: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
            resist: Some(NEffectResist::Standard),
            remote_limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
