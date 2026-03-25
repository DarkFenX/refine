use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NDmgOutputGetter, NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        NEffectDmgKindGetter, NEffectProjMultGetter, NEffectProjOpcSpec,
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
            base: NDmgOutputGetter::MultCharge,
            proj_mult_str: Some(NEffectProjMultGetter::Vorton),
            ..
        }),
        ..
    }
}
