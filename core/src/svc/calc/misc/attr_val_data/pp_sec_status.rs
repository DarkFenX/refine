use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, CalcAttrVals},
    },
    ud::UItemId,
};

pub(super) fn sec_status_postproc_fast(ctx: SvcCtx, item_uid: UItemId, mut val: CalcAttrVals) -> CalcAttrVals {
    let fit_uid = ctx.u_data.items.get(item_uid).dc_ship().unwrap().get_fit_uid();
    let fit = ctx.u_data.fits.get(fit_uid);
    let sec_status = fit.sec_status.into_value();
    val.dogma = sec_status;
    val.extra = sec_status;
    val
}

pub(super) fn sec_status_postproc_info(ctx: SvcCtx, item_uid: UItemId) -> AttrValInfo {
    let fit_uid = ctx.u_data.items.get(item_uid).dc_ship().unwrap().get_fit_uid();
    let fit = ctx.u_data.fits.get(fit_uid);
    let sec_status = fit.sec_status.into_value();
    AttrValInfo::new(sec_status)
}
