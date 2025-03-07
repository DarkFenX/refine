use crate::{
    defs::{EAttrId, OF, SolItemId},
    ec,
    sol::{
        svc::calc::{SolAttrVal, SolAttrValInfo, SolCalc},
        uad::SolUad,
    },
};

pub(in crate::sol::svc::calc) const FTR_COUNT_ATTR: EAttrId = ec::attrs::FTR_SQ_SIZE;

pub(super) fn fighter_count_postproc_fast(
    _calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    mut val: SolAttrVal,
) -> SolAttrVal {
    let count = uad
        .items
        .get_item(item_id)
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
    _calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _info: SolAttrValInfo,
) -> SolAttrValInfo {
    let count = uad
        .items
        .get_item(item_id)
        .unwrap()
        .get_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = OF::from(count.current);
    SolAttrValInfo::new(current_count)
}
