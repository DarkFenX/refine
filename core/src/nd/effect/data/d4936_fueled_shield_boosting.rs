use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectLocalOpcSpec,
        effect::data::shared::base_opc::get_shield_rep_base_opc,
    },
};

const EFFECT_EID: EEffectId = ec::effects::FUELED_SHIELD_BOOSTING;
const EFFECT_AID: AEffectId = ac::effects::FUELED_SHIELD_BOOSTING;

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
            base: get_shield_rep_base_opc,
            limit_attr_id: Some(ac::attrs::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
