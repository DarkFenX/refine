use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc,
        effect::data::shared::opc::get_local_rep_opc,
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
    let extra_mult = if let Some(chargedness) = chargedness
        && let Some(charge_key) = ctx.u_data.items.get(item_key).get_charge_key()
        && ctx.u_data.items.get(charge_key).get_type_id() == ac::items::NANITE_REPAIR_PASTE
        && let Some(rep_mult) = calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().charged_armor_dmg_mult)
    {
        Some((rep_mult - OF(1.0)) * chargedness + OF(1.0))
    } else {
        None
    };
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
