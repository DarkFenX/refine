use crate::{
    ac,
    ad::AAttrId,
    def::OF,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
    ud::UItemKey,
};

pub(in crate::svc::calc) const FTR_COUNT_ATTR: AAttrId = ac::attrs::FTR_SQ_SIZE;

pub(super) fn fighter_count_postproc_fast(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut val: CalcAttrVal,
) -> CalcAttrVal {
    let count = ctx
        .u_data
        .items
        .get(item_key)
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
    ctx: SvcCtx,
    item_key: UItemKey,
    _info: AttrValInfo,
) -> AttrValInfo {
    let count = ctx
        .u_data
        .items
        .get(item_key)
        .get_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = OF::from(count.current);
    AttrValInfo::new(current_count)
}
