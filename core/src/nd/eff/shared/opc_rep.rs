use crate::{
    ac, ad,
    def::{AttrVal, OF},
    misc::Spool,
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        output::{Output, OutputSimple},
    },
    uad::UadItemKey,
};

pub(crate) fn get_local_shield_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UadItemKey,
    a_effect: &ad::AEffectRt,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        a_effect,
        &ac::attrs::SHIELD_BONUS,
        &ac::attrs::SHIELD_CAPACITY,
        true,
    )
}

pub(crate) fn get_local_armor_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UadItemKey,
    a_effect: &ad::AEffectRt,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        a_effect,
        &ac::attrs::ARMOR_DMG_AMOUNT,
        &ac::attrs::ARMOR_HP,
        false,
    )
}

pub(crate) fn get_local_hull_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UadItemKey,
    a_effect: &ad::AEffectRt,
) -> Option<Output<AttrVal>> {
    get_local_rep_opc(
        ctx,
        calc,
        item_key,
        a_effect,
        &ac::attrs::STRUCT_DMG_AMOUNT,
        &ac::attrs::HP,
        false,
    )
}

pub(crate) fn get_remote_shield_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UadItemKey,
    projector_a_effect: &ad::AEffectRt,
    _spool: Option<Spool>,
    projectee_key: Option<UadItemKey>,
) -> Option<Output<AttrVal>> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_key,
        projector_a_effect,
        projectee_key,
        &ac::attrs::SHIELD_BONUS,
        &ac::attrs::SHIELD_CAPACITY,
        true,
    )
}

pub(crate) fn get_remote_armor_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UadItemKey,
    projector_a_effect: &ad::AEffectRt,
    _spool: Option<Spool>,
    projectee_key: Option<UadItemKey>,
) -> Option<Output<AttrVal>> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_key,
        projector_a_effect,
        projectee_key,
        &ac::attrs::ARMOR_DMG_AMOUNT,
        &ac::attrs::ARMOR_HP,
        false,
    )
}

pub(crate) fn get_remote_hull_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UadItemKey,
    projector_a_effect: &ad::AEffectRt,
    _spool: Option<Spool>,
    projectee_key: Option<UadItemKey>,
) -> Option<Output<AttrVal>> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_key,
        projector_a_effect,
        projectee_key,
        &ac::attrs::STRUCT_DMG_AMOUNT,
        &ac::attrs::HP,
        false,
    )
}

pub(crate) fn get_remote_cap_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UadItemKey,
    projector_a_effect: &ad::AEffectRt,
    _spool: Option<Spool>,
    projectee_key: Option<UadItemKey>,
) -> Option<Output<AttrVal>> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_key,
        projector_a_effect,
        projectee_key,
        &ac::attrs::POWER_TRANSFER_AMOUNT,
        &ac::attrs::CAPACITOR_CAPACITY,
        false,
    )
}

fn get_local_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UadItemKey,
    a_effect: &ad::AEffectRt,
    rep_attr_id: &ad::AAttrId,
    limit_attr_id: &ad::AAttrId,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, item_key, rep_attr_id)?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => efuncs::get_effect_duration_s(ctx, calc, item_key, a_effect)?,
    };
    // Total resource pool limit
    if let Some(hp) = get_ship_attr(ctx, calc, item_key, limit_attr_id) {
        amount = amount.min(hp);
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

fn get_remote_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UadItemKey,
    projector_a_effect: &ad::AEffectRt,
    projectee_key: Option<UadItemKey>,
    rep_attr_id: &ad::AAttrId,
    limit_attr_id: &ad::AAttrId,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, rep_attr_id)?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => efuncs::get_effect_duration_s(ctx, calc, projector_key, projector_a_effect)?,
    };
    if let Some(projectee_key) = projectee_key {
        // Effect resistance reduction
        if let Some(rr_mult) =
            efuncs::get_effect_resist_mult(ctx, calc, projector_key, projector_a_effect, projectee_key)
        {
            amount *= rr_mult;
        }
        // Range reduction
        if let Some(proj_mult) =
            efuncs::get_effect_proj_mult(ctx, calc, projector_key, projector_a_effect, projectee_key)
        {
            amount *= proj_mult;
        }
        // Total resource pool limit
        if let Some(hp) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, limit_attr_id) {
            amount = amount.min(hp);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}

fn get_ship_attr(ctx: SvcCtx, calc: &mut Calc, item_key: UadItemKey, a_attr_id: &ad::AAttrId) -> Option<AttrVal> {
    let fit_key = ctx.uad.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.uad.fits.get(fit_key).ship?;
    calc.get_item_attr_val_extra_opt(ctx, ship_key, a_attr_id)
}
