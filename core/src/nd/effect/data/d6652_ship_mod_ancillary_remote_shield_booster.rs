use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectLocalOpcSpec,
        NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::base_opc::{get_ancillary_cap_mult, get_cap_consumer_base_opc, get_shield_rep_base_opc},
    },
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_ANCILLARY_REMOTE_SHIELD_BOOSTER;

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
        outgoing_shield_rep_opc_spec: Some(NEffectProjOpcSpec {
            base: get_shield_rep_base_opc,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeFullStsRestricted),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::SHIELD_CAPACITY),
            ..
        }),
        cap_consume_opc_spec: Some(NEffectLocalOpcSpec {
            base: get_cap_consumer_base_opc,
            charge_mult: Some(get_ancillary_cap_mult),
            ..
        }),
        ..
    }
}
