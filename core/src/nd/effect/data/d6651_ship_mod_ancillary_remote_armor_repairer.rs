use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectProjOpcSpec,
        NEffectResist,
        effect::data::shared::{
            base_opc::{get_ancillary_armor_mult, get_armor_rep_base_opc},
            proj_mult::get_full_noapp_proj_mult,
        },
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
        outgoing_armor_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_armor_rep_base_opc,
            charge_mult: Some(get_ancillary_armor_mult),
            proj_mult_str: Some(get_full_noapp_proj_mult),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
