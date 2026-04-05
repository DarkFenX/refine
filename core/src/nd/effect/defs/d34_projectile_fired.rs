use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectDmgKindGetter,
        NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec,
    },
};

const EFFECT_EID: EEffectId = EEffectId::PROJECTILE_FIRED;
const EFFECT_AID: AEffectId = AEffectId::PROJECTILE_FIRED;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        dmg_kind: Some(NEffectDmgKindGetter::Turret),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::MultCharge,
            proj_mult_str: Some(NEffectProjGetter::Turret),
            ..
        }),
        ..
    }
}
