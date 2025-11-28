use crate::{
    ac,
    ad::AAttrId,
    def::{AttrVal, OF},
    misc::{EffectSpec, Spool},
    nd::{NProjMultGetter, NSpoolResolver},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::eff) fn get_local_shield_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        r_effect,
        &ac::attrs::SHIELD_BONUS,
        &ac::attrs::SHIELD_CAPACITY,
        true,
    )
}

pub(in crate::nd::eff) fn get_local_armor_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        r_effect,
        &ac::attrs::ARMOR_DMG_AMOUNT,
        &ac::attrs::ARMOR_HP,
        false,
    )
}

pub(in crate::nd::eff) fn get_local_hull_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        r_effect,
        &ac::attrs::STRUCT_DMG_AMOUNT,
        &ac::attrs::HP,
        false,
    )
}

fn get_local_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
    rep_attr_id: &AAttrId,
    limit_attr_id: &AAttrId,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, item_key, rep_attr_id)?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, item_key, r_effect)?,
    };
    // Total resource pool limit
    if let Some(hp) = get_ship_attr(ctx, calc, item_key, limit_attr_id) {
        amount = amount.min(hp);
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

pub(in crate::nd::eff) fn get_outgoing_shield_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        &ac::attrs::SHIELD_BONUS,
        &ac::attrs::SHIELD_CAPACITY,
        true,
    )
}

pub(in crate::nd::eff) fn get_outgoing_armor_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        &ac::attrs::ARMOR_DMG_AMOUNT,
        &ac::attrs::ARMOR_HP,
        false,
    )
}

pub(in crate::nd::eff) fn get_outgoing_hull_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        &ac::attrs::STRUCT_DMG_AMOUNT,
        &ac::attrs::HP,
        false,
    )
}

pub(in crate::nd::eff) fn get_outgoing_cap_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<AttrVal>> {
    get_outgoing_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        spool_resolver,
        projectee_key,
        proj_mult_getter,
        &ac::attrs::POWER_TRANSFER_AMOUNT,
        &ac::attrs::CAPACITOR_CAPACITY,
        false,
    )
}

fn get_outgoing_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    spool_resolver: Option<NSpoolResolver>,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
    amount_attr_id: &AAttrId,
    limit_attr_id: &AAttrId,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, amount_attr_id)?;
    if let Some(spool_resolver) = spool_resolver
        && let Some(resolved_spool) = spool_resolver(ctx, calc, projector_key, projector_effect, spool)
    {
        amount *= resolved_spool.mult;
    }
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, projector_key, projector_effect)?,
    };
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        amount *= proj_mult_getter(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        if let Some(rr_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            amount *= rr_mult;
        }
        // Total resource pool limit
        if let Some(hp) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, limit_attr_id) {
            amount = amount.min(hp);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

fn get_ship_attr(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, a_attr_id: &AAttrId) -> Option<AttrVal> {
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_attr_val_extra_opt(ctx, ship_key, a_attr_id)
}
