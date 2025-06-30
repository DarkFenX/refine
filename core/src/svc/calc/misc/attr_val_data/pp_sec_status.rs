use crate::{
    ac, ad,
    def::ItemKey,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
};

pub(in crate::svc::calc) const SEC_STATUS_ATTR: ad::AAttrId = ac::attrs::PILOT_SECURITY_STATUS;

pub(super) fn sec_status_postproc_fast(
    _calc: &mut Calc,
    ctx: &SvcCtx,
    item_key: ItemKey,
    mut val: CalcAttrVal,
) -> CalcAttrVal {
    let fit_key = ctx.uad.items.get(item_key).get_ship().unwrap().get_fit_key();
    let fit = ctx.uad.fits.get(fit_key);
    val.dogma = fit.sec_status.get_inner();
    val.extra = fit.sec_status.get_inner();
    val
}

pub(super) fn sec_status_postproc_info(
    _calc: &mut Calc,
    ctx: &SvcCtx,
    item_key: ItemKey,
    _info: AttrValInfo,
) -> AttrValInfo {
    let fit_key = ctx.uad.items.get(item_key).get_ship().unwrap().get_fit_key();
    let fit = ctx.uad.fits.get(fit_key);
    AttrValInfo::new(fit.sec_status.get_inner())
}
