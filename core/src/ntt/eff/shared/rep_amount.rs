use crate::{
    ac, ad,
    def::{AttrVal, ItemKey},
    svc::{SvcCtx, calc::Calc},
};

pub(crate) fn get_local_shield_rep_amount(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
    let mut amount = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::SHIELD_BONUS)?;
    // If rep target is defined and has less than repped amount HP, limit by total HP
    if let Some(hp) = get_ship_attr(ctx, calc, item_key, &ac::attrs::SHIELD_CAPACITY) {
        amount = amount.min(hp);
    }
    Some(amount)
}

pub(crate) fn get_local_armor_rep_amount(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
    let mut amount = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::ARMOR_DMG_AMOUNT)?;
    // If rep target is defined and has less than repped amount HP, limit by total HP
    if let Some(hp) = get_ship_attr(ctx, calc, item_key, &ac::attrs::ARMOR_HP) {
        amount = amount.min(hp);
    }
    Some(amount)
}

fn get_ship_attr(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> Option<AttrVal> {
    let fit_key = ctx.uad.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.uad.fits.get(fit_key).ship?;
    calc.get_item_attr_val_extra(ctx, ship_key, a_attr_id)
}
