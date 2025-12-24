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

const E_EFFECT_ID: EEffectId = ec::effects::FUELED_SHIELD_BOOSTING;
const A_EFFECT_ID: AEffectId = ac::effects::FUELED_SHIELD_BOOSTING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate {
                can_run_uncharged: true,
            })),
            activates_charge: false,
        }),
        local_shield_rep_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_shield_rep_base_opc,
            ilimit_attr_id: Some(ac::attrs::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
