use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::calc::{AttrValInfo, Calc, CalcAttrVal},
        uad::Uad,
    },
};

pub(in crate::sol::svc::calc) const SEC_STATUS_ATTR: ad::AAttrId = ac::attrs::PILOT_SECURITY_STATUS;

pub(super) fn sec_status_postproc_fast(
    _calc: &mut Calc,
    uad: &Uad,
    item_key: ItemKey,
    mut val: CalcAttrVal,
) -> CalcAttrVal {
    let fit_key = uad.items.get(item_key).get_ship().unwrap().get_fit_key();
    let fit = uad.fits.get(fit_key);
    val.dogma = fit.sec_status;
    val.extra = fit.sec_status;
    val
}

pub(super) fn sec_status_postproc_info(
    _calc: &mut Calc,
    uad: &Uad,
    item_key: ItemKey,
    _info: AttrValInfo,
) -> AttrValInfo {
    let fit_key = uad.items.get(item_key).get_ship().unwrap().get_fit_key();
    let fit = uad.fits.get(fit_key);
    AttrValInfo::new(fit.sec_status)
}
