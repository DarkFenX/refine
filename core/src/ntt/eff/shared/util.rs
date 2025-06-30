use crate::{def::ItemKey, svc::SvcCtx};

pub(in crate::ntt::eff) fn get_item_fit_ship_key(ctx: &SvcCtx, item_key: ItemKey) -> Option<ItemKey> {
    let item = ctx.uad.items.get(item_key);
    let fit_key = item.get_fit_key()?;
    let fit = ctx.uad.fits.get(fit_key);
    fit.ship
}
