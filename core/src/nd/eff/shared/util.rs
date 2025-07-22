use crate::{svc::SvcCtx, uad::UadItemKey};

pub(in crate::nd::eff) fn get_item_fit_ship_key(ctx: SvcCtx, item_key: UadItemKey) -> Option<UadItemKey> {
    let item = ctx.uad.items.get(item_key);
    let fit_key = item.get_fit_key()?;
    let fit = ctx.uad.fits.get(fit_key);
    fit.ship
}
