use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, ResolvedSpool, Spool},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
            spool::get_resolved_spool,
        },
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::TGT_DISINTEGRATOR_ATTACK;
const A_EFFECT_ID: AEffectId = ac::effects::TGT_DISINTEGRATOR_ATTACK;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
            spool_resolver: Some(internal_get_resolved_spool),
            normal_dmg_opc_getter: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
    spool: Option<Spool>,
) -> Option<ResolvedSpool> {
    get_resolved_spool(
        ctx,
        calc,
        item_key,
        r_effect,
        spool,
        &ac::attrs::DMG_MULT_BONUS_PER_CYCLE,
        &ac::attrs::DMG_MULT_BONUS_MAX,
    )
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_r_effect: &REffect,
    spool: Option<Spool>,
    _projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let projector_u_item = ctx.u_data.items.get(projector_key);
    let charge_key = projector_u_item.get_charge_key()?;
    let dmg_mult = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DMG_MULT)?;
    let mut dmg_em = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::EM_DMG)?;
    let mut dmg_therm = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::THERM_DMG)?;
    let mut dmg_kin = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::KIN_DMG)?;
    let mut dmg_expl = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::EXPL_DMG)?;
    if let Some(resolved_spool) = internal_get_resolved_spool(ctx, calc, projector_key, projector_r_effect, spool) {
        dmg_em *= resolved_spool.mult;
        dmg_therm *= resolved_spool.mult;
        dmg_kin *= resolved_spool.mult;
        dmg_expl *= resolved_spool.mult;
    }
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: dmg_em * dmg_mult,
            thermal: dmg_therm * dmg_mult,
            kinetic: dmg_kin * dmg_mult,
            explosive: dmg_expl * dmg_mult,
        },
        delay: OF(0.0),
    }))
}
