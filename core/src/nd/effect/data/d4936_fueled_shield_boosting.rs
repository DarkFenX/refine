use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectLocalOpcSpec,
        NGeneralOutputGetter, effect::data::shared::base_opc::get_ancillary_cap_mult,
    },
};

const EFFECT_EID: EEffectId = EEffectId::FUELED_SHIELD_BOOSTING;
const EFFECT_AID: AEffectId = AEffectId::FUELED_SHIELD_BOOSTING;

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
        local_shield_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: NGeneralOutputGetter::RepShield,
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        cap_consume_opc_spec: Some(NEffectLocalOpcSpec {
            base: NGeneralOutputGetter::CapConsumer,
            charge_mult: Some(get_ancillary_cap_mult),
            ..
        }),
        ..
    }
}
