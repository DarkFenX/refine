use crate::{svc::SvcCtx, ud::UItemId};

pub(in crate::nd::effect::data) fn get_item_fit_ship_uid(ctx: SvcCtx, item_uid: UItemId) -> Option<UItemId> {
    let item = ctx.u_data.items.get(item_uid);
    let fit_uid = item.get_fit_uid()?;
    let fit = ctx.u_data.fits.get(fit_uid);
    fit.ship
}
