use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectDmgKindGetter,
        NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectSpoolAttrs,
    },
};

const EFFECT_AID: AEffectId = AEffectId::TGT_DISINTEGRATOR_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        spool_attrs: Some(NEffectSpoolAttrs {
            step_attr_id: AAttrId::DMG_MULT_BONUS_PER_CYCLE,
            max_attr_id: AAttrId::DMG_MULT_BONUS_MAX,
        }),
        dmg_kind: Some(NEffectDmgKindGetter::Turret),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::MultCharge,
            spoolable: true,
            proj_mult_str: Some(NEffectProjGetter::Disintegrator),
            ..
        }),
        ..
    }
}
