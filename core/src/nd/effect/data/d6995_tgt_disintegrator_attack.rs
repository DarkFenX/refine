use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectDmgKind,
        NEffectProjOpcSpec, NSpoolAttrs,
        effect::data::shared::{
            base_opc::get_instant_charge_mult_dmg_base_opc, proj_mult::get_disintegrator_proj_mult,
        },
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::TGT_DISINTEGRATOR_ATTACK;
const A_EFFECT_ID: AEffectId = ac::effects::TGT_DISINTEGRATOR_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        spool_attr_ids: Some(NSpoolAttrs {
            step: ac::attrs::DMG_MULT_BONUS_PER_CYCLE,
            max: ac::attrs::DMG_MULT_BONUS_MAX,
        }),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_charge_mult_dmg_base_opc,
            spoolable: true,
            proj_mult_str: Some(get_disintegrator_proj_mult),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Turret
}
