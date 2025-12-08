use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
    ud::UItemKey,
};

pub(super) fn sec_status_postproc_fast(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut val: CalcAttrVal,
) -> CalcAttrVal {
    let fit_key = ctx.u_data.items.get(item_key).dc_ship().unwrap().get_fit_key();
    let fit = ctx.u_data.fits.get(fit_key);
    val.dogma = fit.sec_status.get_inner();
    val.extra = fit.sec_status.get_inner();
    val
}

pub(super) fn sec_status_postproc_info(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    _info: AttrValInfo,
) -> AttrValInfo {
    let fit_key = ctx.u_data.items.get(item_key).dc_ship().unwrap().get_fit_key();
    let fit = ctx.u_data.fits.get(fit_key);
    AttrValInfo::new(fit.sec_status.get_inner())
}
