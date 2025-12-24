use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectLocalOpcSpec,
        effect::data::shared::opc::{get_local_shield_rep_base_opc, get_local_shield_rep_ilimit},
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
            base: get_local_shield_rep_base_opc,
            instance_limit: Some(get_local_shield_rep_ilimit),
            ..
        }),
        ..
    }
}
