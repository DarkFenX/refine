use crate::{
    ac, ad,
    sol::{
        ItemId,
        svc::calc::{AttrValInfo, Calc, CalcAttrVal},
        uad::Uad,
    },
};

pub(in crate::sol::svc::calc) const SEC_STATUS_ATTR: ad::AAttrId = ac::attrs::PILOT_SECURITY_STATUS;

pub(super) fn sec_status_postproc_fast(
    _calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    mut val: CalcAttrVal,
) -> CalcAttrVal {
    let fit_id = uad.items.get_by_id(item_id).unwrap().get_ship().unwrap().get_fit_id();
    let fit = uad.fits.get_fit(&fit_id).unwrap();
    val.dogma = fit.sec_status;
    val.extra = fit.sec_status;
    val
}

pub(super) fn sec_status_postproc_info(
    _calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    _info: AttrValInfo,
) -> AttrValInfo {
    let fit_id = uad.items.get_by_id(item_id).unwrap().get_ship().unwrap().get_fit_id();
    let fit = uad.fits.get_fit(&fit_id).unwrap();
    AttrValInfo::new(fit.sec_status)
}
