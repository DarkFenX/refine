use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        ItemId,
        svc::calc::{AttrValInfo, Calc, CalcAttrVal},
        uad::Uad,
    },
};

pub(in crate::sol::svc::calc) const FTR_COUNT_ATTR: ad::AAttrId = ac::attrs::FTR_SQ_SIZE;

pub(super) fn fighter_count_postproc_fast(
    _calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    mut val: CalcAttrVal,
) -> CalcAttrVal {
    let count = uad
        .items
        .get_by_id(item_id)
        .unwrap()
        .get_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = OF::from(count.current);
    val.dogma = current_count;
    val.extra = current_count;
    val
}

pub(super) fn fighter_count_postproc_info(
    _calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    _info: AttrValInfo,
) -> AttrValInfo {
    let count = uad
        .items
        .get_by_id(item_id)
        .unwrap()
        .get_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = OF::from(count.current);
    AttrValInfo::new(current_count)
}
