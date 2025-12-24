use crate::{
    def::AttrVal,
    rd::RAttrKey,
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(in crate::nd::effect::data) fn get_self_shield_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_self_ilimit(ctx, calc, item_key, ctx.ac().shield_capacity)
}
pub(in crate::nd::effect::data) fn get_proj_shield_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_proj_ilimit(ctx, calc, item_key, ctx.ac().shield_capacity)
}

pub(in crate::nd::effect::data) fn get_self_armor_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_self_ilimit(ctx, calc, item_key, ctx.ac().armor_hp)
}
pub(in crate::nd::effect::data) fn get_proj_armor_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_proj_ilimit(ctx, calc, item_key, ctx.ac().armor_hp)
}

pub(in crate::nd::effect::data) fn get_self_hull_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_self_ilimit(ctx, calc, item_key, ctx.ac().hp)
}
pub(in crate::nd::effect::data) fn get_proj_hull_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_proj_ilimit(ctx, calc, item_key, ctx.ac().hp)
}

pub(in crate::nd::effect::data) fn get_proj_cap_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<AttrVal> {
    get_proj_ilimit(ctx, calc, item_key, ctx.ac().capacitor_capacity)
}

fn get_self_ilimit(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, attr_key: Option<RAttrKey>) -> Option<AttrVal> {
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_oattr_oextra(ctx, ship_key, attr_key)
}
fn get_proj_ilimit(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, attr_key: Option<RAttrKey>) -> Option<AttrVal> {
    calc.get_item_oattr_oextra(ctx, item_key, attr_key)
}
