use crate::{
    ac, ad,
    def::{AttrVal, ItemKey},
    misc::{EffectSpec, Spool},
    svc::{SvcCtx, calc::Calc, get_proj_mult, get_resist_mult},
};

pub(crate) fn get_local_shield_rep_amount(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
    get_local_rep_amount(
        ctx,
        calc,
        item_key,
        &ac::attrs::SHIELD_BONUS,
        &ac::attrs::SHIELD_CAPACITY,
    )
}

pub(crate) fn get_local_armor_rep_amount(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
    get_local_rep_amount(ctx, calc, item_key, &ac::attrs::ARMOR_DMG_AMOUNT, &ac::attrs::ARMOR_HP)
}

pub(crate) fn get_local_hull_rep_amount(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
    get_local_rep_amount(ctx, calc, item_key, &ac::attrs::STRUCT_DMG_AMOUNT, &ac::attrs::HP)
}

pub(crate) fn get_remote_shield_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    _spool: Option<Spool>,
    projectee_key: Option<ItemKey>,
) -> Option<AttrVal> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_espec,
        projectee_key,
        &ac::attrs::SHIELD_BONUS,
        &ac::attrs::SHIELD_CAPACITY,
    )
}

pub(crate) fn get_remote_armor_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    _spool: Option<Spool>,
    projectee_key: Option<ItemKey>,
) -> Option<AttrVal> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_espec,
        projectee_key,
        &ac::attrs::ARMOR_DMG_AMOUNT,
        &ac::attrs::ARMOR_HP,
    )
}

pub(crate) fn get_remote_hull_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    _spool: Option<Spool>,
    projectee_key: Option<ItemKey>,
) -> Option<AttrVal> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_espec,
        projectee_key,
        &ac::attrs::STRUCT_DMG_AMOUNT,
        &ac::attrs::HP,
    )
}

pub(crate) fn get_remote_cap_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    _spool: Option<Spool>,
    projectee_key: Option<ItemKey>,
) -> Option<AttrVal> {
    get_remote_rep_amount(
        ctx,
        calc,
        projector_espec,
        projectee_key,
        &ac::attrs::POWER_TRANSFER_AMOUNT,
        &ac::attrs::CAPACITOR_CAPACITY,
    )
}

fn get_local_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    rep_attr_id: &ad::AAttrId,
    limit_attr_id: &ad::AAttrId,
) -> Option<AttrVal> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, item_key, rep_attr_id)?;
    // Total resource pool limit
    if let Some(hp) = get_ship_attr(ctx, calc, item_key, limit_attr_id) {
        amount = amount.min(hp);
    }
    Some(amount)
}

fn get_remote_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    projectee_key: Option<ItemKey>,
    rep_attr_id: &ad::AAttrId,
    limit_attr_id: &ad::AAttrId,
) -> Option<AttrVal> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_espec.item_key, rep_attr_id)?;
    if let Some(projectee_key) = projectee_key {
        // Effect resistance reduction
        if let Some(rr_mult) = get_resist_mult(ctx, calc, &projector_espec, projectee_key) {
            amount *= rr_mult;
        }
        // Range reduction
        if let Some(proj_mult) = get_proj_mult(ctx, calc, projector_espec, projectee_key) {
            amount *= proj_mult;
        }
        // Total resource pool limit
        if let Some(hp) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, limit_attr_id) {
            amount = amount.min(hp);
        }
    }
    Some(amount)
}

fn get_ship_attr(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> Option<AttrVal> {
    let fit_key = ctx.uad.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.uad.fits.get(fit_key).ship?;
    calc.get_item_attr_val_extra_opt(ctx, ship_key, a_attr_id)
}
