use crate::{
    ad::{AAttrId, AEffectId},
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

const EFFECT_EID: EEffectId = EEffectId::TGT_DISINTEGRATOR_ATTACK;
const EFFECT_AID: AEffectId = AEffectId::TGT_DISINTEGRATOR_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        spool_attrs: Some(NSpoolAttrs {
            step_attr_id: AAttrId::DMG_MULT_BONUS_PER_CYCLE,
            max_attr_id: AAttrId::DMG_MULT_BONUS_MAX,
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
