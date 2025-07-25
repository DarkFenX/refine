use crate::{svc::SvcCtx, ud::UItemKey};

pub(in crate::nd::eff) fn get_item_fit_ship_key(ctx: SvcCtx, item_key: UItemKey) -> Option<UItemKey> {
    let item = ctx.u_data.items.get(item_key);
    let fit_key = item.get_fit_key()?;
    let fit = ctx.u_data.fits.get(fit_key);
    fit.ship
}
