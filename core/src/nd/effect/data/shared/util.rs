use crate::{svc::SvcCtx, ud::UItemId};

pub(in crate::nd::effect::data) fn get_item_fit_ship_key(ctx: SvcCtx, item_key: UItemId) -> Option<UItemId> {
    let item = ctx.u_data.items.get(item_key);
    let fit_key = item.get_fit_uid()?;
    let fit = ctx.u_data.fits.get(fit_key);
    fit.ship
}
