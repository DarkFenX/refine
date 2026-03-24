use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectDmgKindGetter,
        NEffectProjMultGetter, NEffectProjOpcSpec,
        effect::data::shared::base_opc::get_instant_charge_mult_dmg_base_opc,
    },
};

const EFFECT_EID: EEffectId = EEffectId::CHAIN_LIGHTNING;
const EFFECT_AID: AEffectId = AEffectId::CHAIN_LIGHTNING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        dmg_kind_getter: Some(NEffectDmgKindGetter::Vorton),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_charge_mult_dmg_base_opc,
            proj_mult_str: Some(NEffectProjMultGetter::Vorton),
            ..
        }),
        ..
    }
}
