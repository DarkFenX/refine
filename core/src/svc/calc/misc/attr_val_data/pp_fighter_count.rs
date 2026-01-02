use crate::{
    def::AttrVal,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals},
    },
    ud::UItemId,
};

pub(super) fn fighter_count_postproc_fast(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    mut val: CalcAttrVals,
) -> CalcAttrVals {
    let count = ctx
        .u_data
        .items
        .get(item_key)
        .dc_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = AttrVal::from(count.current);
    val.dogma = current_count;
    val.extra = current_count;
    val
}

pub(super) fn fighter_count_postproc_info(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    _info: AttrValInfo,
) -> AttrValInfo {
    let count = ctx
        .u_data
        .items
        .get(item_key)
        .dc_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = AttrVal::from(count.current);
    AttrValInfo::new(current_count)
}
