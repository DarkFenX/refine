use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectChargeMultGetter, NEffectGeneralOutputGetter, NEffectLocalOpcSpec, NEffectProjGetter,
        NEffectProjOpcSpec, NEffectResist,
    },
};

const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate {
                can_run_uncharged: true,
            })),
            activates_charge: false,
        }),
        outgoing_shield_rep: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::RepShield,
            proj_mult_str: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
            resist: Some(NEffectResist::Standard),
            remote_limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
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
