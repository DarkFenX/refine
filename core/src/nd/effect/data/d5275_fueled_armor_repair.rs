use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        effect::data::shared::opc::{get_ancillary_armor_mult, get_local_rep_opc},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::FUELED_ARMOR_REPAIR;
const A_EFFECT_ID: AEffectId = ac::effects::FUELED_ARMOR_REPAIR;

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
        local_armor_rep_opc_getter: Some(internal_get_local_rep_opc),
        ..
    }
}

fn internal_get_local_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    chargedness: Option<AttrVal>,
) -> Option<Output<AttrVal>> {
    let extra_mult = get_ancillary_armor_mult(ctx, calc, item_key, chargedness);
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        effect,
        ctx.ac().armor_dmg_amount,
        ctx.ac().armor_hp,
        extra_mult,
        false,
    )
}
