use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_shield_rep_base_opc, proj_mult::get_full_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER;

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
        outgoing_shield_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_shield_rep_base_opc,
            proj_mult: get_full_noapp_proj_mult,
            ilimit_attr_id: Some(ac::attrs::SHIELD_CAPACITY),
            ..
        }),
        ..
    }
}
