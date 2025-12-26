use super::shared::process_mult;
use crate::{
    def::AttrVal,
    rd::{RAttrKey, REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

pub(super) struct LocalInvariantData<T>
where
    T: Copy,
{
    pub(super) base: Output<T>,
    pub(super) ilimit: Option<AttrVal>,
}

pub(super) fn try_make_local_inv_data<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<LocalInvariantData<T>>
where
    T: Copy,
{
    Some(LocalInvariantData {
        base: (ospec.base)(ctx, calc, item_key, effect)?,
        ilimit: get_ship_limit(ctx, calc, item_key, ospec.ilimit_attr_key),
    })
}

fn get_ship_limit(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, attr_key: Option<RAttrKey>) -> Option<AttrVal> {
    let attr_key = attr_key?;
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_attr_oextra(ctx, ship_key, attr_key)
        .and_then(|v| process_mult(v))
}
