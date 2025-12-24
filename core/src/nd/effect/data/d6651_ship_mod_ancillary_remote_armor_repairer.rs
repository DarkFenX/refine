use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::{get_ancillary_armor_mult, get_armor_rep_base_opc},
            ilimit::get_proj_armor_ilimit,
            proj_mult::get_full_noapp_proj_mult,
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER;

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
        outgoing_armor_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_armor_rep_base_opc,
            proj_mult: get_full_noapp_proj_mult,
            charge_mult: Some(get_ancillary_armor_mult),
            instance_limit: Some(get_proj_armor_ilimit),
            ..
        }),
        ..
    }
}
